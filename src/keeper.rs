use crate::{
    borrowers::{VaultMap, Borrowers},
    escalator::GeometricGasPrice,
    liquidations::{AuctionMap, Liquidator},
    Result,
};

use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{collections::HashMap, io::Write, path::PathBuf, sync::Arc};
use tracing::{debug_span};

#[serde_as]
#[derive(Serialize, Deserialize, Default)]
/// The state which is stored in our logs
pub struct State {
    /// The auctions being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    auctions: AuctionMap,
    /// The borrowers being monitored
    #[serde_as(as = "Vec<(_, _)>")]
    vaults: VaultMap,
    /// The last observed block
    last_block: u64,
}

/// The keeper monitors the chain for both liquidation opportunities and for
/// participation in auctions using Uniswap as a liquidity source
pub struct Keeper<M> {
    client: Arc<M>,
    last_block: U64,

    borrowers: Borrowers<M>,
    liquidator: Liquidator<M>,
}

impl<M: Middleware> Keeper<M> {
    /// Instantiates the keeper. `state` should be passed if there is previous
    /// data which should be taken into account from a previous run
    pub async fn new(
        client: Arc<M>,
        controller: Address,
        liquidations: Address,
        flashloan: Address,
        multicall: Option<Address>,
        min_ratio: u16,
        gas_escalator: GeometricGasPrice,
        state: Option<State>,
    ) -> Result<Keeper<M>, M> {
        let (vaults, auctions, last_block) = match state {
            Some(state) => (state.vaults, state.auctions, state.last_block.into()),
            None => (HashMap::new(), HashMap::new(), 0.into()),
        };

        let borrowers = Borrowers::new(controller, liquidations, multicall, client.clone(), vaults).await;
        let liquidator = Liquidator::new(
            controller,
            liquidations,
            flashloan,
            multicall,
            min_ratio,
            client.clone(),
            auctions,
            gas_escalator,
        )
        .await;

        Ok(Self {
            client,
            borrowers,
            liquidator,
            last_block,
        })
    }

    pub async fn run(&mut self, fname: PathBuf, start_block: Option<u64>) -> Result<(), M> {
        // Create the initial list of borrowers from the start_block, if provided
        if let Some(start_block) = start_block {
            self.last_block = start_block.into();
        }

        let watcher = self.client.clone();
        let mut on_block = watcher
            .watch_blocks()
            .await
            .map_err(ContractError::MiddlewareError)?
            .stream();

        let mut file: Option<std::fs::File> = None;
        while on_block.next().await.is_some() {
            let block_number = self
                .client
                .get_block_number()
                .await
                .map_err(ContractError::MiddlewareError)?;

            if block_number % 10 == 0.into() {
                // on each new block we open a new file handler to dump our state.
                // we should just have a database connection instead here...
                file = Some(
                    std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&fname)
                        .unwrap(),
                );
            }

            let span = debug_span!("eloop", block = %block_number);
            let _enter = span.enter();

            // run the logic for this block
            self.on_block(block_number).await?;

            // update our last block
            self.last_block = block_number;

            // Log once every 10 blocks
            if let Some(file) = file.take() {
                self.log(file);
            }
        }

        Ok(())
    }

    /// Runs the liquidation business logic for the specified block
    async fn on_block(&mut self, block_number: U64) -> Result<(), M> {
        // Get the gas price - TODO: Replace with gas price oracle
        let gas_price = self
            .client
            .get_gas_price()
            .await
            .map_err(ContractError::MiddlewareError)?;

        // 1. Check if our transactions have been mined
        self.liquidator.remove_or_bump().await?;

        // 2. update our dataset with the new block's data
        self.borrowers
            .update_vaults(self.last_block, block_number)
            .await?;

        // 3. trigger the auction for any undercollateralized borrowers
        self.liquidator
            .start_auctions(self.borrowers.vaults.iter(), gas_price)
            .await?;

        // 4. try buying the ones which are worth buying
        self.liquidator
            .buy_opportunities(self.last_block, block_number, gas_price)
            .await?;
        Ok(())
    }

    fn log<W: Write>(&self, w: W) {
        serde_json::to_writer(
            w,
            &State {
                auctions: self.liquidator.auctions.clone(),
                vaults: self.borrowers.vaults.clone(),
                last_block: self.last_block.as_u64(),
            },
        )
        .unwrap();
    }
}