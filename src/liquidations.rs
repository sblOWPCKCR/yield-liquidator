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

    /// The minimum ratio (collateral/debt) to trigger liquidation
    min_ratio: u16,

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
    under_auction: bool,
    /// The debt which can be repaid
    debt: u128,
    /// The collateral which can be seized
    collateral: u128,

    ratio_pct: u16,
    is_at_minimal_price: bool,

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
        min_ratio: u16,
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
            min_ratio,
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
            let is_still_valid: bool = self.buy(vault_id, Instant::now(), gas_price).await?;
            if !is_still_valid {
                self.auctions.remove(&vault_id);
            }
        }

        Ok(())
    }

    /// Tries to buy the collateral associated with a user's liquidation auction
    /// via a flashloan funded by Uniswap.
    ///
    /// Returns
    ///  - Result<false>: auction is no longer valid, we need to forget about it
    ///  - Result<true>: auction is still valid
    async fn buy(&mut self, vault_id: VaultIdType, now: Instant, gas_price: U256) -> Result<bool, M> {
        // only iterate over users that do not have active auctions
        if let Some(pending_tx) = self.pending_auctions.get(&vault_id) {
            trace!(tx_hash = ?pending_tx.1, vault_id=?vault_id, "bid not confirmed yet");
            return Ok(true);
        }

        // Get the vault's info
        let auction = self.get_auction(vault_id).await?;

        if !auction.under_auction {
            debug!(vault_id=?vault_id, auction=?auction, "Auction is no longer active");
            return Ok(false);
        }

        // Skip auctions which do not have any outstanding debt
        if auction.debt == 0 {
            debug!(vault_id=?vault_id, auction=?auction, "Has no debt - skipping");
            return Ok(true);
        }

        let mut buy: bool = false;
        if auction.ratio_pct <= self.min_ratio {
            debug!(vault_id=?vault_id, auction=?auction,
                ratio=auction.ratio_pct, ratio_threshold=self.min_ratio,
                "Ratio threshold is reached, buying");
            buy = true;
        }
        if auction.is_at_minimal_price {
            debug!(vault_id=?vault_id, auction=?auction,
                ratio=auction.ratio_pct, ratio_threshold=self.min_ratio,
                "Is at minimal price, buying");
            buy = true;
        }
        if !buy {
            debug!(vault_id=?vault_id, auction=?auction, "Not time to buy yet");
            return Ok(true);
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

        Ok(true)
    }

    /// Triggers liquidations for any vulnerable positions which were fetched from the
    /// controller
    pub async fn start_auctions(
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
                    "found undercollateralized vault. starting an auction",
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
            .add_call(auction_fn)
            ;

        let ((_, series_id, ilk_id), (art, ink), (auction_owner, auction_start)):
            ((Address, SeriesIdType, InkIdType), (u128, u128), (Address, u32)) = multicall.call().await?;

        let (_, art_id, _) = self.cauldron.series(series_id).call().await?;

        // resolve asset IDs
        let multicall = self
            .multicall
            .clear_calls()
            .add_call(self.cauldron.assets(art_id))
            .add_call(self.cauldron.assets(ilk_id))
            .add_call(self.pairflash.is_at_minimal_price(vault_id, ilk_id))
            .add_call(self.pairflash.collateral_to_debt_ratio(vault_id, series_id, art_id, ilk_id, art))
            ;

        let (art_address, ink_address, is_at_minimal_price, ratio_u256): (Address, Address, bool, U256) = multicall.call().await?;

        let ratio_pct_u256 = ratio_u256 / U256::exp10(16);
        let ratio_pct: u16 = {
            if ratio_pct_u256 > U256::from(u16::MAX) {
                error!(vault_id=?vault_id, ratio_pct_u256=?ratio_pct_u256, "Ratio is too big");
                0
            } else {
                (ratio_pct_u256.as_u64()) as u16
            }
        };

        Ok(Auction {
            under_auction: (auction_owner != Address::zero()),
            started: auction_start,
            collateral: ink,
            debt: art,
            ratio_pct: ratio_pct,
            is_at_minimal_price: is_at_minimal_price,
            debt_id: art_id,
            collateral_id: ilk_id,
            debt_address: art_address,
            collateral_address: ink_address
        })

    }
}
