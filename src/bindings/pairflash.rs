pub use pairflash_mod::*;
#[allow(clippy::too_many_arguments)]
mod pairflash_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "PairFlash was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PAIRFLASH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\"},{\"internalType\":\"contract ISwapRouter\",\"name\":\"_swapRouter\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_factory\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_WETH9\",\"type\":\"address\"},{\"internalType\":\"contract IWitch\",\"name\":\"_witch\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes12\",\"name\":\"vaultId\",\"type\":\"bytes12\"},{\"internalType\":\"bytes6\",\"name\":\"seriesId\",\"type\":\"bytes6\"},{\"internalType\":\"bytes6\",\"name\":\"baseId\",\"type\":\"bytes6\"},{\"internalType\":\"bytes6\",\"name\":\"ilkId\",\"type\":\"bytes6\"},{\"internalType\":\"uint128\",\"name\":\"art\",\"type\":\"uint128\"}],\"name\":\"collateralToDebtRatio\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"collateral\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"debt\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"debtAmount\",\"type\":\"uint256\"},{\"internalType\":\"bytes12\",\"name\":\"vaultId\",\"type\":\"bytes12\"},{\"internalType\":\"bytes6\",\"name\":\"collateralId\",\"type\":\"bytes6\"},{\"internalType\":\"bytes6\",\"name\":\"debtId\",\"type\":\"bytes6\"},{\"internalType\":\"bytes6\",\"name\":\"seriesId\",\"type\":\"bytes6\"}],\"internalType\":\"struct PairFlash.FlashParams\",\"name\":\"params\",\"type\":\"tuple\"}],\"name\":\"initFlash\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes12\",\"name\":\"vaultId\",\"type\":\"bytes12\"},{\"internalType\":\"bytes6\",\"name\":\"ilkId\",\"type\":\"bytes6\"}],\"name\":\"isAtMinimalPrice\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"refundETH\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"swapRouter\",\"outputs\":[{\"internalType\":\"contract ISwapRouter\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"sweepToken\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"fee1\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"uniswapV3FlashCallback\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"unwrapWETH9\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct PairFlash<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PairFlash<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PairFlash<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PairFlash))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PairFlash<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), PAIRFLASH_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `WETH9` (0x4aa4a4fc) function"]
        pub fn weth9(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateralToDebtRatio` (0x21335c10) function"]
        pub fn collateral_to_debt_ratio(
            &self,
            vault_id: [u8; 12],
            series_id: [u8; 6],
            base_id: [u8; 6],
            ilk_id: [u8; 6],
            art: u128,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [33, 51, 92, 16],
                    (vault_id, series_id, base_id, ilk_id, art),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initFlash` (0xb758ebbf) function"]
        pub fn init_flash(
            &self,
            params: FlashParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 88, 235, 191], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAtMinimalPrice` (0x6ee98e5e) function"]
        pub fn is_at_minimal_price(
            &self,
            vault_id: [u8; 12],
            ilk_id: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([110, 233, 142, 94], (vault_id, ilk_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refundETH` (0x12210e8a) function"]
        pub fn refund_eth(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapRouter` (0xc31c9c07) function"]
        pub fn swap_router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([195, 28, 156, 7], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepToken` (0xdf2ab5bb) function"]
        pub fn sweep_token(
            &self,
            token: ethers::core::types::Address,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV3FlashCallback` (0xe9cbafb0) function"]
        pub fn uniswap_v3_flash_callback(
            &self,
            fee_0: ethers::core::types::U256,
            fee_1: ethers::core::types::U256,
            data: Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 203, 175, 176], (fee_0, fee_1, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9` (0x49404b7c) function"]
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "`FlashParams(address,address,uint256,bytes12,bytes6,bytes6,bytes6)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType)]
    pub struct FlashParams {
        pub collateral: ethers::core::types::Address,
        pub debt: ethers::core::types::Address,
        pub debt_amount: ethers::core::types::U256,
        pub vault_id: [u8; 12],
        pub collateral_id: [u8; 6],
        pub debt_id: [u8; 6],
        pub series_id: [u8; 6],
    }
}
