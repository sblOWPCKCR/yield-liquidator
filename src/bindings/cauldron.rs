pub use cauldron_mod::*;
#[allow(clippy::too_many_arguments)]
mod cauldron_mod {
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
    #[doc = "Cauldron was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CAULDRON_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"assetId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"AssetAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint96\",\n        \"name\": \"max\",\n        \"type\": \"uint96\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint24\",\n        \"name\": \"min\",\n        \"type\": \"uint24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"dec\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"DebtLimitsSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"IlkAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"oracle\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RateOracleAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"newAdminRole\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"RoleAdminChanged\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleGranted\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleRevoked\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"SeriesAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"rateAtMaturity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"SeriesMatured\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"oracle\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint32\",\n        \"name\": \"ratio\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"name\": \"SpotOracleAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"VaultBuilt\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"VaultDestroyed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"receiver\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"VaultGiven\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int128\",\n        \"name\": \"ink\",\n        \"type\": \"int128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int128\",\n        \"name\": \"art\",\n        \"type\": \"int128\"\n      }\n    ],\n    \"name\": \"VaultPoured\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"VaultRolled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"from\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"to\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"ink\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"VaultStirred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"VaultTweaked\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"LOCK\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"LOCK8605463013\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ROOT\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ROOT4146650865\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"accrual\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"assetId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addAsset\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6[]\",\n        \"name\": \"ilkIds\",\n        \"type\": \"bytes6[]\"\n      }\n    ],\n    \"name\": \"addIlks\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"contract IFYToken\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addSeries\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"assets\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"balances\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"ink\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"build\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"seriesId\",\n            \"type\": \"bytes6\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"ilkId\",\n            \"type\": \"bytes6\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Vault\",\n        \"name\": \"vault\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"debt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint96\",\n        \"name\": \"max\",\n        \"type\": \"uint96\"\n      },\n      {\n        \"internalType\": \"uint24\",\n        \"name\": \"min\",\n        \"type\": \"uint24\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"dec\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"sum\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"base\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"debtFromBase\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"debtToBase\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"base\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"destroy\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"getRoleAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"receiver\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"give\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"seriesId\",\n            \"type\": \"bytes6\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"ilkId\",\n            \"type\": \"bytes6\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Vault\",\n        \"name\": \"vault\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"receiver\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grab\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4[]\",\n        \"name\": \"roles\",\n        \"type\": \"bytes4[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRoles\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasRole\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"ilks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"lendingOracles\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IOracle\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"level\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"lockRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"mature\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"int128\",\n        \"name\": \"ink\",\n        \"type\": \"int128\"\n      },\n      {\n        \"internalType\": \"int128\",\n        \"name\": \"art\",\n        \"type\": \"int128\"\n      }\n    ],\n    \"name\": \"pour\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"art\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"ink\",\n            \"type\": \"uint128\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Balances\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"ratesAtMaturity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"renounceRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4[]\",\n        \"name\": \"roles\",\n        \"type\": \"bytes4[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRoles\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"newSeriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"int128\",\n        \"name\": \"art\",\n        \"type\": \"int128\"\n      }\n    ],\n    \"name\": \"roll\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"seriesId\",\n            \"type\": \"bytes6\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"ilkId\",\n            \"type\": \"bytes6\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Vault\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"art\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"ink\",\n            \"type\": \"uint128\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Balances\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"series\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IFYToken\",\n        \"name\": \"fyToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"maturity\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"uint96\",\n        \"name\": \"max\",\n        \"type\": \"uint96\"\n      },\n      {\n        \"internalType\": \"uint24\",\n        \"name\": \"min\",\n        \"type\": \"uint24\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"dec\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"setDebtLimits\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"contract IOracle\",\n        \"name\": \"oracle\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setLendingOracle\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"adminRole\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"setRoleAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"baseId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"contract IOracle\",\n        \"name\": \"oracle\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"ratio\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"name\": \"setSpotOracle\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"ink\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"slurp\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"art\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"ink\",\n            \"type\": \"uint128\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Balances\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"spotOracles\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IOracle\",\n        \"name\": \"oracle\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"ratio\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"from\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"to\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"ink\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"art\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"stir\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"art\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"ink\",\n            \"type\": \"uint128\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Balances\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"art\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"ink\",\n            \"type\": \"uint128\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Balances\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"tweak\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"owner\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"seriesId\",\n            \"type\": \"bytes6\"\n          },\n          {\n            \"internalType\": \"bytes6\",\n            \"name\": \"ilkId\",\n            \"type\": \"bytes6\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Vault\",\n        \"name\": \"vault\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"vaults\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"seriesId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct Cauldron<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Cauldron<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Cauldron<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Cauldron))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Cauldron<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), CAULDRON_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `LOCK` (0xa4f0d7d0) function"]
        pub fn lock(&self) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([164, 240, 215, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LOCK8605463013` (0xffffffff) function"]
        pub fn lock8605463013(&self) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([255, 255, 255, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT` (0x5909c12f) function"]
        pub fn root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([89, 9, 193, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT4146650865` (0x00000000) function"]
        pub fn root4146650865(&self) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([0, 0, 0, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accrual` (0x97389c3a) function"]
        pub fn accrual(
            &self,
            series_id: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([151, 56, 156, 58], series_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addAsset` (0xe665f9e4) function"]
        pub fn add_asset(
            &self,
            asset_id: [u8; 6],
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 101, 249, 228], (asset_id, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addIlks` (0x53eecb77) function"]
        pub fn add_ilks(
            &self,
            series_id: [u8; 6],
            ilk_ids: ::std::vec::Vec<[u8; 6]>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 238, 203, 119], (series_id, ilk_ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addSeries` (0x96dcdf26) function"]
        pub fn add_series(
            &self,
            series_id: [u8; 6],
            base_id: [u8; 6],
            fy_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 220, 223, 38], (series_id, base_id, fy_token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assets` (0x19be8d12) function"]
        pub fn assets(
            &self,
            p0: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([25, 190, 141, 18], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balances` (0x7229280c) function"]
        pub fn balances(
            &self,
            p0: [u8; 12],
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([114, 41, 40, 12], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `build` (0x272c0a6e) function"]
        pub fn build(
            &self,
            owner: ethers::core::types::Address,
            vault_id: [u8; 12],
            series_id: [u8; 6],
            ilk_id: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, [u8; 6], [u8; 6]),
        > {
            self.0
                .method_hash([39, 44, 10, 110], (owner, vault_id, series_id, ilk_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debt` (0x9be24c4d) function"]
        pub fn debt(
            &self,
            p0: [u8; 6],
            p1: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, (u128, u32, u8, u128)> {
            self.0
                .method_hash([155, 226, 76, 77], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debtFromBase` (0x9dd6c00e) function"]
        pub fn debt_from_base(
            &self,
            series_id: [u8; 6],
            base: u128,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([157, 214, 192, 14], (series_id, base))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debtToBase` (0x550b0253) function"]
        pub fn debt_to_base(
            &self,
            series_id: [u8; 6],
            art: u128,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([85, 11, 2, 83], (series_id, art))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `destroy` (0x782006b6) function"]
        pub fn destroy(
            &self,
            vault_id: [u8; 12],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 32, 6, 182], vault_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x5ba5e9f0) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([91, 165, 233, 240], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `give` (0x798a828b) function"]
        pub fn give(
            &self,
            vault_id: [u8; 12],
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, [u8; 6], [u8; 6]),
        > {
            self.0
                .method_hash([121, 138, 130, 139], (vault_id, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grab` (0x93ea8747) function"]
        pub fn grab(
            &self,
            vault_id: [u8; 12],
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 234, 135, 71], (vault_id, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0xde02cde7) function"]
        pub fn grant_role(
            &self,
            role: [u8; 4],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 2, 205, 231], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRoles` (0xeffae353) function"]
        pub fn grant_roles(
            &self,
            roles: ::std::vec::Vec<[u8; 4]>,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 250, 227, 83], (roles, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x10ab9432) function"]
        pub fn has_role(
            &self,
            role: [u8; 4],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([16, 171, 148, 50], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ilks` (0x415d8ad8) function"]
        pub fn ilks(
            &self,
            p0: [u8; 6],
            p1: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([65, 93, 138, 216], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lendingOracles` (0xb2bb4cc7) function"]
        pub fn lending_oracles(
            &self,
            p0: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([178, 187, 76, 199], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `level` (0xf4135771) function"]
        pub fn level(
            &self,
            vault_id: [u8; 12],
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([244, 19, 87, 113], vault_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lockRole` (0x559742d9) function"]
        pub fn lock_role(&self, role: [u8; 4]) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 151, 66, 217], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mature` (0x8e8a5a29) function"]
        pub fn mature(
            &self,
            series_id: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 138, 90, 41], series_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pour` (0xae8ad492) function"]
        pub fn pour(
            &self,
            vault_id: [u8; 12],
            ink: i128,
            art: i128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([174, 138, 212, 146], (vault_id, ink, art))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ratesAtMaturity` (0x2fcdab66) function"]
        pub fn rates_at_maturity(
            &self,
            p0: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([47, 205, 171, 102], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x687f0e4c) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 4],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 127, 14, 76], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0x44faded0) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 4],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 250, 222, 208], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRoles` (0xad82110f) function"]
        pub fn revoke_roles(
            &self,
            roles: ::std::vec::Vec<[u8; 4]>,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 130, 17, 15], (roles, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roll` (0x6295afac) function"]
        pub fn roll(
            &self,
            vault_id: [u8; 12],
            new_series_id: [u8; 6],
            art: i128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                (ethers::core::types::Address, [u8; 6], [u8; 6]),
                (u128, u128),
            ),
        > {
            self.0
                .method_hash([98, 149, 175, 172], (vault_id, new_series_id, art))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `series` (0x55d03e34) function"]
        pub fn series(
            &self,
            p0: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::Address, [u8; 6], u32)>
        {
            self.0
                .method_hash([85, 208, 62, 52], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDebtLimits` (0xd3fc152b) function"]
        pub fn set_debt_limits(
            &self,
            base_id: [u8; 6],
            ilk_id: [u8; 6],
            max: u128,
            min: u32,
            dec: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 252, 21, 43], (base_id, ilk_id, max, min, dec))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLendingOracle` (0x55727d48) function"]
        pub fn set_lending_oracle(
            &self,
            base_id: [u8; 6],
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 114, 125, 72], (base_id, oracle))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRoleAdmin` (0xae93c1b5) function"]
        pub fn set_role_admin(
            &self,
            role: [u8; 4],
            admin_role: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 147, 193, 181], (role, admin_role))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSpotOracle` (0x8ef6bdfa) function"]
        pub fn set_spot_oracle(
            &self,
            base_id: [u8; 6],
            ilk_id: [u8; 6],
            oracle: ethers::core::types::Address,
            ratio: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 246, 189, 250], (base_id, ilk_id, oracle, ratio))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slurp` (0x2b7b312f) function"]
        pub fn slurp(
            &self,
            vault_id: [u8; 12],
            ink: u128,
            art: u128,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([43, 123, 49, 47], (vault_id, ink, art))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `spotOracles` (0x43d8f0bb) function"]
        pub fn spot_oracles(
            &self,
            p0: [u8; 6],
            p1: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::Address, u32)>
        {
            self.0
                .method_hash([67, 216, 240, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stir` (0x2458028a) function"]
        pub fn stir(
            &self,
            from: [u8; 12],
            to: [u8; 12],
            ink: u128,
            art: u128,
        ) -> ethers::contract::builders::ContractCall<M, ((u128, u128), (u128, u128))> {
            self.0
                .method_hash([36, 88, 2, 138], (from, to, ink, art))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tweak` (0xb9af3920) function"]
        pub fn tweak(
            &self,
            vault_id: [u8; 12],
            series_id: [u8; 6],
            ilk_id: [u8; 6],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, [u8; 6], [u8; 6]),
        > {
            self.0
                .method_hash([185, 175, 57, 32], (vault_id, series_id, ilk_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vaults` (0x1e81f829) function"]
        pub fn vaults(
            &self,
            p0: [u8; 12],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, [u8; 6], [u8; 6]),
        > {
            self.0
                .method_hash([30, 129, 248, 41], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AssetAdded` event"]
        pub fn asset_added_filter(&self) -> ethers::contract::builders::Event<M, AssetAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DebtLimitsSet` event"]
        pub fn debt_limits_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DebtLimitsSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IlkAdded` event"]
        pub fn ilk_added_filter(&self) -> ethers::contract::builders::Event<M, IlkAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RateOracleAdded` event"]
        pub fn rate_oracle_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RateOracleAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SeriesAdded` event"]
        pub fn series_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SeriesAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SeriesMatured` event"]
        pub fn series_matured_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SeriesMaturedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SpotOracleAdded` event"]
        pub fn spot_oracle_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SpotOracleAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultBuilt` event"]
        pub fn vault_built_filter(&self) -> ethers::contract::builders::Event<M, VaultBuiltFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultDestroyed` event"]
        pub fn vault_destroyed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VaultDestroyedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultGiven` event"]
        pub fn vault_given_filter(&self) -> ethers::contract::builders::Event<M, VaultGivenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultPoured` event"]
        pub fn vault_poured_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VaultPouredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultRolled` event"]
        pub fn vault_rolled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VaultRolledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultStirred` event"]
        pub fn vault_stirred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VaultStirredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VaultTweaked` event"]
        pub fn vault_tweaked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VaultTweakedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CauldronEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "AssetAdded", abi = "AssetAdded(bytes6,address)")]
    pub struct AssetAddedFilter {
        #[ethevent(indexed)]
        pub asset_id: [u8; 6],
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(
        name = "DebtLimitsSet",
        abi = "DebtLimitsSet(bytes6,bytes6,uint96,uint24,uint8)"
    )]
    pub struct DebtLimitsSetFilter {
        #[ethevent(indexed)]
        pub base_id: [u8; 6],
        #[ethevent(indexed)]
        pub ilk_id: [u8; 6],
        pub max: u128,
        pub min: u32,
        pub dec: u8,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "IlkAdded", abi = "IlkAdded(bytes6,bytes6)")]
    pub struct IlkAddedFilter {
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        #[ethevent(indexed)]
        pub ilk_id: [u8; 6],
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "RateOracleAdded", abi = "RateOracleAdded(bytes6,address)")]
    pub struct RateOracleAddedFilter {
        #[ethevent(indexed)]
        pub base_id: [u8; 6],
        #[ethevent(indexed)]
        pub oracle: ethers::core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "RoleAdminChanged", abi = "RoleAdminChanged(bytes4,bytes4)")]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 4],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 4],
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes4,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 4],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes4,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 4],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "SeriesAdded", abi = "SeriesAdded(bytes6,bytes6,address)")]
    pub struct SeriesAddedFilter {
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        #[ethevent(indexed)]
        pub base_id: [u8; 6],
        #[ethevent(indexed)]
        pub fy_token: ethers::core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "SeriesMatured", abi = "SeriesMatured(bytes6,uint256)")]
    pub struct SeriesMaturedFilter {
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        pub rate_at_maturity: ethers::core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(
        name = "SpotOracleAdded",
        abi = "SpotOracleAdded(bytes6,bytes6,address,uint32)"
    )]
    pub struct SpotOracleAddedFilter {
        #[ethevent(indexed)]
        pub base_id: [u8; 6],
        #[ethevent(indexed)]
        pub ilk_id: [u8; 6],
        #[ethevent(indexed)]
        pub oracle: ethers::core::types::Address,
        pub ratio: u32,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "VaultBuilt", abi = "VaultBuilt(bytes12,address,bytes6,bytes6)")]
    pub struct VaultBuiltFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        pub ilk_id: [u8; 6],
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "VaultDestroyed", abi = "VaultDestroyed(bytes12)")]
    pub struct VaultDestroyedFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "VaultGiven", abi = "VaultGiven(bytes12,address)")]
    pub struct VaultGivenFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub receiver: ethers::core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(
        name = "VaultPoured",
        abi = "VaultPoured(bytes12,bytes6,bytes6,int128,int128)"
    )]
    pub struct VaultPouredFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        #[ethevent(indexed)]
        pub ilk_id: [u8; 6],
        pub ink: i128,
        pub art: i128,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "VaultRolled", abi = "VaultRolled(bytes12,bytes6,uint128)")]
    pub struct VaultRolledFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        pub art: u128,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(
        name = "VaultStirred",
        abi = "VaultStirred(bytes12,bytes12,uint128,uint128)"
    )]
    pub struct VaultStirredFilter {
        #[ethevent(indexed)]
        pub from: [u8; 12],
        #[ethevent(indexed)]
        pub to: [u8; 12],
        pub ink: u128,
        pub art: u128,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent)]
    #[ethevent(name = "VaultTweaked", abi = "VaultTweaked(bytes12,bytes6,bytes6)")]
    pub struct VaultTweakedFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub series_id: [u8; 6],
        #[ethevent(indexed)]
        pub ilk_id: [u8; 6],
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CauldronEvents {
        AssetAddedFilter(AssetAddedFilter),
        DebtLimitsSetFilter(DebtLimitsSetFilter),
        IlkAddedFilter(IlkAddedFilter),
        RateOracleAddedFilter(RateOracleAddedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        SeriesAddedFilter(SeriesAddedFilter),
        SeriesMaturedFilter(SeriesMaturedFilter),
        SpotOracleAddedFilter(SpotOracleAddedFilter),
        VaultBuiltFilter(VaultBuiltFilter),
        VaultDestroyedFilter(VaultDestroyedFilter),
        VaultGivenFilter(VaultGivenFilter),
        VaultPouredFilter(VaultPouredFilter),
        VaultRolledFilter(VaultRolledFilter),
        VaultStirredFilter(VaultStirredFilter),
        VaultTweakedFilter(VaultTweakedFilter),
    }
    impl ethers::core::abi::Tokenizable for CauldronEvents {
        fn from_token(
            token: ethers::core::abi::Token,
        ) -> Result<Self, ethers::core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetAddedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::AssetAddedFilter(decoded));
            }
            if let Ok(decoded) = DebtLimitsSetFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::DebtLimitsSetFilter(decoded));
            }
            if let Ok(decoded) = IlkAddedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::IlkAddedFilter(decoded));
            }
            if let Ok(decoded) = RateOracleAddedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::RateOracleAddedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SeriesAddedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::SeriesAddedFilter(decoded));
            }
            if let Ok(decoded) = SeriesMaturedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::SeriesMaturedFilter(decoded));
            }
            if let Ok(decoded) = SpotOracleAddedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::SpotOracleAddedFilter(decoded));
            }
            if let Ok(decoded) = VaultBuiltFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultBuiltFilter(decoded));
            }
            if let Ok(decoded) = VaultDestroyedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultDestroyedFilter(decoded));
            }
            if let Ok(decoded) = VaultGivenFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultGivenFilter(decoded));
            }
            if let Ok(decoded) = VaultPouredFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultPouredFilter(decoded));
            }
            if let Ok(decoded) = VaultRolledFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultRolledFilter(decoded));
            }
            if let Ok(decoded) = VaultStirredFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultStirredFilter(decoded));
            }
            if let Ok(decoded) = VaultTweakedFilter::from_token(token.clone()) {
                return Ok(CauldronEvents::VaultTweakedFilter(decoded));
            }
            Err(ethers::core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers::core::abi::Token {
            match self {
                CauldronEvents::AssetAddedFilter(element) => element.into_token(),
                CauldronEvents::DebtLimitsSetFilter(element) => element.into_token(),
                CauldronEvents::IlkAddedFilter(element) => element.into_token(),
                CauldronEvents::RateOracleAddedFilter(element) => element.into_token(),
                CauldronEvents::RoleAdminChangedFilter(element) => element.into_token(),
                CauldronEvents::RoleGrantedFilter(element) => element.into_token(),
                CauldronEvents::RoleRevokedFilter(element) => element.into_token(),
                CauldronEvents::SeriesAddedFilter(element) => element.into_token(),
                CauldronEvents::SeriesMaturedFilter(element) => element.into_token(),
                CauldronEvents::SpotOracleAddedFilter(element) => element.into_token(),
                CauldronEvents::VaultBuiltFilter(element) => element.into_token(),
                CauldronEvents::VaultDestroyedFilter(element) => element.into_token(),
                CauldronEvents::VaultGivenFilter(element) => element.into_token(),
                CauldronEvents::VaultPouredFilter(element) => element.into_token(),
                CauldronEvents::VaultRolledFilter(element) => element.into_token(),
                CauldronEvents::VaultStirredFilter(element) => element.into_token(),
                CauldronEvents::VaultTweakedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers::core::abi::TokenizableItem for CauldronEvents {}
    impl ethers::contract::EthLogDecode for CauldronEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetAddedFilter::decode_log(log) {
                return Ok(CauldronEvents::AssetAddedFilter(decoded));
            }
            if let Ok(decoded) = DebtLimitsSetFilter::decode_log(log) {
                return Ok(CauldronEvents::DebtLimitsSetFilter(decoded));
            }
            if let Ok(decoded) = IlkAddedFilter::decode_log(log) {
                return Ok(CauldronEvents::IlkAddedFilter(decoded));
            }
            if let Ok(decoded) = RateOracleAddedFilter::decode_log(log) {
                return Ok(CauldronEvents::RateOracleAddedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(CauldronEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(CauldronEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(CauldronEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = SeriesAddedFilter::decode_log(log) {
                return Ok(CauldronEvents::SeriesAddedFilter(decoded));
            }
            if let Ok(decoded) = SeriesMaturedFilter::decode_log(log) {
                return Ok(CauldronEvents::SeriesMaturedFilter(decoded));
            }
            if let Ok(decoded) = SpotOracleAddedFilter::decode_log(log) {
                return Ok(CauldronEvents::SpotOracleAddedFilter(decoded));
            }
            if let Ok(decoded) = VaultBuiltFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultBuiltFilter(decoded));
            }
            if let Ok(decoded) = VaultDestroyedFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultDestroyedFilter(decoded));
            }
            if let Ok(decoded) = VaultGivenFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultGivenFilter(decoded));
            }
            if let Ok(decoded) = VaultPouredFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultPouredFilter(decoded));
            }
            if let Ok(decoded) = VaultRolledFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultRolledFilter(decoded));
            }
            if let Ok(decoded) = VaultStirredFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultStirredFilter(decoded));
            }
            if let Ok(decoded) = VaultTweakedFilter::decode_log(log) {
                return Ok(CauldronEvents::VaultTweakedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    #[doc = "`Balances(uint128,uint128)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType)]
    pub struct Balances {
        pub art: u128,
        pub ink: u128,
    }
    #[doc = "`Vault(address,bytes6,bytes6)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType)]
    pub struct Vault {
        pub owner: ethers::core::types::Address,
        pub series_id: [u8; 6],
        pub ilk_id: [u8; 6],
    }
}
