//! Auctions Module
//!
//! This module is responsible for triggering and participating in a Auction's
//! dutch auction
use crate::{
    bindings::{Cauldron, Witch, ArtIdType, SeriesIdType, InkIdType, VaultIdType, PairFlash, FlashParams},
    borrowers::{Vault},
    escalator::GeometricGasPrice,
    merge, Result,
};

use ethers_core::types::transaction::eip2718::TypedTransaction;

use ethers::{
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, sync::Arc, time::Instant};
use tracing::{debug, debug_span, error, info, trace};

pub type AuctionMap = HashMap<VaultIdType, Auction>;


#[derive(Clone)]
pub struct Liquidator<M> {
    cauldron: Cauldron<M>,
    liquidator: Witch<M>,
    pairflash: PairFlash<M>,

    /// The currently active auctions
    pub auctions: HashMap<VaultIdType, Auction>,

    /// We use multicall to batch together calls and have reduced stress on
    /// our RPC endpoint
    multicall: Multicall<M>,

    /// The minimum profit to be extracted per liquidation
    min_profit: U256,

    pending_liquidations: HashMap<VaultIdType, PendingTransaction>,
    pending_auctions: HashMap<VaultIdType, PendingTransaction>,
    gas_escalator: GeometricGasPrice,
}

/// Tx / Hash/ Submitted at time
type PendingTransaction = (TypedTransaction, TxHash, Instant);

/// An initiated auction
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Auction {
    /// The start time of the auction
    started: u32,
    /// The debt which can be repaid
    debt: u128,
    /// The collateral which can be seized
    collateral: u128,

    debt_id: ArtIdType,
    collateral_id: InkIdType,

    debt_address: Address,
    collateral_address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum TxType {
    Auction,
    Liquidation,
}

impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TxType::Auction => "auction",
            TxType::Liquidation => "liquidation",
        };
        write!(f, "{}", string)
    }
}

impl<M: Middleware> Liquidator<M> {
    /// Constructor
    pub async fn new(
        cauldron: Address,
        liquidator: Address,
        flashloan: Address,
        multicall: Option<Address>,
        min_profit: U256,
        client: Arc<M>,
        auctions: HashMap<VaultIdType, Auction>,
        gas_escalator: GeometricGasPrice,
    ) -> Self {
        let multicall = Multicall::new(client.clone(), multicall)
            .await
            .expect("could not initialize multicall");

        Self {
            cauldron: Cauldron::new(cauldron, client.clone()),
            liquidator: Witch::new(liquidator, client.clone()),
            pairflash: PairFlash::new(flashloan, client.clone()),
            multicall,
            min_profit,
            auctions,

            pending_liquidations: HashMap::new(),
            pending_auctions: HashMap::new(),
            gas_escalator,
        }
    }

    /// Checks if any transactions which have been submitted are mined, removes
    /// them if they were successful, otherwise bumps their gas price
    pub async fn remove_or_bump(&mut self) -> Result<(), M> {
        let now = Instant::now();

        let liquidator_client = self.liquidator.client();
        // Check all the pending liquidations
        Liquidator::remove_or_bump_inner(now, liquidator_client, &self.gas_escalator,
            &mut self.pending_liquidations, "liquidations").await?;
        Liquidator::remove_or_bump_inner(now, liquidator_client, &self.gas_escalator,
            &mut self.pending_auctions, "auctions").await?;

        Ok(())
    }

    async fn remove_or_bump_inner<K: Clone + Eq + ::std::hash::Hash + std::fmt::Debug>(
        now: Instant,
        client: &M,
        gas_escalator: &GeometricGasPrice,
        pending_txs: &mut HashMap<K, PendingTransaction>,
        tx_type: &str
        ) -> Result<(), M> {
        for (addr, (pending_tx_wrapper, tx_hash, instant)) in pending_txs.clone().into_iter() {
            let pending_tx = match pending_tx_wrapper {
                TypedTransaction::Legacy(x) => x,
                _ => panic!("Non-legacy transactions are not supported yet")
            };
            debug_assert!(
                pending_tx.gas_price.is_some(),
                "gas price must be set in pending txs"
            );

            // get the receipt and check inclusion, or bump its gas price
            let receipt = client
                .get_transaction_receipt(tx_hash)
                .await
                .map_err(ContractError::MiddlewareError)?;
            if let Some(receipt) = receipt {
                pending_txs.remove(&addr);
                let status = if receipt.status == Some(1.into()) {
                    "success"
                } else {
                    "fail"
                };
                trace!(tx_hash = ?tx_hash, gas_used = %receipt.gas_used.unwrap_or_default(), user = ?addr,
                    status = status, tx_type, "confirmed");
            } else {
                // Get the new gas price based on how much time passed since the
                // tx was last broadcast
                let new_gas_price = gas_escalator.get_gas_price(
                    pending_tx.gas_price.expect("gas price must be set"),
                    now.duration_since(instant).as_secs(),
                );

                let replacement_tx = pending_txs
                    .get_mut(&addr)
                    .expect("tx will always be found since we're iterating over the map");

                // bump the gas price
                if let TypedTransaction::Legacy(x) = &mut replacement_tx.0 {
                    x.gas_price = Some(new_gas_price);
                } else {
                    panic!("Non-legacy transactions are not supported yet");
                }

                // rebroadcast (TODO: Can we avoid cloning?)
                replacement_tx.1 = *client
                    .send_transaction(replacement_tx.0.clone(), None)
                    .await
                    .map_err(ContractError::MiddlewareError)?;

                trace!(tx_hash = ?tx_hash, new_gas_price = %new_gas_price, user = ?addr,
                    tx_type, "replaced");
            }
        }

        Ok(())
    }

    /// Sends a bid for any of the liquidation auctions.
    pub async fn buy_opportunities(
        &mut self,
        from_block: U64,
        to_block: U64,
        gas_price: U256,
    ) -> Result<(), M> {
        let all_auctions = {
            let liquidations = self
                .liquidator
                .auctioned_filter()
                .from_block(from_block)
                .to_block(to_block)
                .query()
                .await?;
            let new_liquidations = liquidations
                .iter()
                .map(|x| x.vault_id).collect::<Vec<_>>();
            merge(new_liquidations, &self.auctions)
        };

        for vault_id in all_auctions {
            self.buy(vault_id, Instant::now(), gas_price).await?;
        }

        Ok(())
    }

    /// Tries to buy the collateral associated with a user's liquidation auction
    /// via a flashloan funded by Uniswap's DAI/WETH pair.
    async fn buy(&mut self, vault_id: VaultIdType, now: Instant, gas_price: U256) -> Result<(), M> {
        // only iterate over users that do not have active auctions
        if let Some(pending_tx) = self.pending_auctions.get(&vault_id) {
            trace!(tx_hash = ?pending_tx.1, vault_id=?vault_id, "bid not confirmed yet");
            return Ok(());
        }

        // Get the vault's info
        let auction = self.get_auction(vault_id).await?;
        // Skip auctions which do not have any outstanding debt
        if auction.debt == 0 {
            debug!(vault_id=?vault_id, auction=?auction, "Has no debt - skipping");
            return Ok(());
        }

        if self.auctions.insert(vault_id, auction.clone()).is_none() {
            debug!(vault_id=?vault_id, auction=?auction, "new auction");
        }
        let span = debug_span!("buying", vault_id=?vault_id, auction=?auction);
        let _enter = span.enter();

        let args = FlashParams {
            collateral: auction.collateral_address,
            debt: auction.debt_address,
            debt_amount: U256::from(auction.debt),
            vault_id: vault_id,
            collateral_id: auction.collateral_id,
            debt_id: auction.debt_id
        };
        let call = self.pairflash.init_flash(args)
            .legacy() // XXX
            .gas_price(gas_price)
            .block(BlockNumber::Pending);

        let tx = call.tx.clone();

        match call.send().await {
            Ok(hash) => {
                // record the tx
                trace!(tx_hash = ?hash, "Submitted buy order");
                self.pending_auctions
                    .entry(vault_id)
                    .or_insert((tx, *hash, now));
            }
            Err(err) => {
                let err = err.to_string();
                error!("Error: {}", err);
            }
        };

        Ok(())
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn trigger_liquidations(
        &mut self,
        vaults: impl Iterator<Item = (&VaultIdType, &Vault)>,
        gas_price: U256,
    ) -> Result<(), M> {
        debug!("checking for undercollateralized positions...");

        let now = Instant::now();

        for (vault_id, vault) in vaults {
            debug!(vault_id=?vault_id, "Checking vault");
            // only iterate over vaults that do not have pending liquidations
            if let Some(pending_tx) = self.pending_liquidations.get(vault_id) {
                trace!(tx_hash = ?pending_tx.1, vault_id = ?vault_id, "liquidation not confirmed yet");
                continue;
            }

            if !vault.is_collateralized {
                if vault.under_auction {
                    debug!(vault_id = ?vault_id, details = ?vault, "found vault under auction, ignoring it");
                    continue;
                }
                info!(
                    vault_id = ?vault_id, details = ?vault,
                    "found undercollateralized vault. triggering liquidation",
                );

                // Send the tx and track it
                // XXX legacy
                let call = self.liquidator.auction(*vault_id).legacy().gas_price(gas_price);
                let tx = call.tx.clone();
                let tx_hash = call.send().await?;
                trace!(tx_hash = ?tx_hash, vault_id = ?vault_id, "Submitted liquidation");
                self.pending_liquidations
                    .entry(*vault_id)
                    .or_insert((tx, *tx_hash, now));
            } else {
                debug!(vault_id=?vault_id, "Vault is collateralized");
            }
        }
        debug!("all done");
        Ok(())
    }

    async fn get_auction(&mut self, vault_id: VaultIdType) -> Result<Auction, M> {
        // re-fetch asset ids. we shouldn't be buying too often -> OK to not optimize this
        // we could pass Borrowers::vaults map to Liquidators to get rid of the extra lookup
        let asset_ids_fn = self.cauldron.vaults(vault_id);
        let balances_fn = self.cauldron.balances(vault_id);
        let auction_fn = self.liquidator.auctions(vault_id);

        let multicall = self
            .multicall
            .clear_calls()
            .add_call(asset_ids_fn)
            .add_call(balances_fn)
            .add_call(auction_fn);

        let ((_, series_id, ilk_id), (art, ink), (_auction_owner, auction_start)):
            ((Address, SeriesIdType, InkIdType), (u128, u128), (Address, u32)) = multicall.call().await?;

        let (_, art_id, _) = self.cauldron.series(series_id).call().await?;

        // resolve asset IDs
        let multicall = self
            .multicall
            .clear_calls()
            .add_call(self.cauldron.assets(art_id))
            .add_call(self.cauldron.assets(ilk_id))
            ;

        let (art_address, ink_address): (Address, Address) = multicall.call().await?;

        Ok(Auction {
            started: auction_start,
            collateral: ink,
            debt: art,
            debt_id: art_id,
            collateral_id: ilk_id,
            debt_address: art_address,
            collateral_address: ink_address
        })

    }
}
