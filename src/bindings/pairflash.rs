pub use pairflash_mod::*;
#[allow(clippy::too_many_arguments)]
mod pairflash_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers_contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers_core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers_providers::Middleware;
    #[doc = "PairFlash was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PAIRFLASH_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract ISwapRouter\",\n        \"name\": \"_swapRouter\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_factory\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_WETH9\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IWitch\",\n        \"name\": \"_witch\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH9\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collateralToDebtRatio\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"collateral\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"debt\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"debtAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes12\",\n            \"name\": \"vaultId\",\n            \"type\": \"bytes12\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"collateralId\",\n            \"type\": \"bytes6\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"debtId\",\n            \"type\": \"bytes6\"\n          }\n        ],\n        \"internalType\": \"struct PairFlash.FlashParams\",\n        \"name\": \"params\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"initFlash\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"isAtMinimalPrice\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"refundETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"swapRouter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ISwapRouter\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountMinimum\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"sweepToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee1\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"uniswapV3FlashCallback\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountMinimum\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"unwrapWETH9\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct PairFlash<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for PairFlash<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for PairFlash<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PairFlash))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> PairFlash<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), PAIRFLASH_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `WETH9` (0x4aa4a4fc) function"]
        pub fn weth9(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
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
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
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
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initFlash` (0x195545ff) function"]
        pub fn init_flash(
            &self,
            params: FlashParams,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 85, 69, 255], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAtMinimalPrice` (0x6ee98e5e) function"]
        pub fn is_at_minimal_price(
            &self,
            vault_id: [u8; 12],
            ilk_id: [u8; 6],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([110, 233, 142, 94], (vault_id, ilk_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refundETH` (0x12210e8a) function"]
        pub fn refund_eth(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapRouter` (0xc31c9c07) function"]
        pub fn swap_router(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([195, 28, 156, 7], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepToken` (0xdf2ab5bb) function"]
        pub fn sweep_token(
            &self,
            token: ethers_core::types::Address,
            amount_minimum: ethers_core::types::U256,
            recipient: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV3FlashCallback` (0xe9cbafb0) function"]
        pub fn uniswap_v3_flash_callback(
            &self,
            fee_0: ethers_core::types::U256,
            fee_1: ethers_core::types::U256,
            data: Vec<u8>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 203, 175, 176], (fee_0, fee_1, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9` (0x49404b7c) function"]
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ethers_core::types::U256,
            recipient: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "`FlashParams(address,address,uint256,bytes12,bytes6,bytes6)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthAbiType)]
    pub struct FlashParams {
        pub collateral: ethers_core::types::Address,
        pub debt: ethers_core::types::Address,
        pub debt_amount: ethers_core::types::U256,
        pub vault_id: [u8; 12],
        pub collateral_id: [u8; 6],
        pub debt_id: [u8; 6],
    }
}
