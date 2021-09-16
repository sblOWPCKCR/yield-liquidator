pub use witch_mod::*;
#[allow(clippy::too_many_arguments)]
mod witch_mod {
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
    #[doc = "Witch was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WITCH_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> = ethers_contract::Lazy::new(
        || {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract ICauldron\",\n        \"name\": \"cauldron_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract ILadle\",\n        \"name\": \"ladle_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"start\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Auctioned\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"buyer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"ink\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"art\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Bought\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint32\",\n        \"name\": \"duration\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"initialOffer\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"dust\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"IlkSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"newAdminRole\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"RoleAdminChanged\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleGranted\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleRevoked\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"LOCK\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ROOT\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"auction\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"\",\n        \"type\": \"bytes12\"\n      }\n    ],\n    \"name\": \"auctions\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"start\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"base\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"min\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"buy\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"ink\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"cauldron\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ICauldron\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"getRoleAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4[]\",\n        \"name\": \"roles\",\n        \"type\": \"bytes4[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRoles\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasRole\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"\",\n        \"type\": \"bytes6\"\n      }\n    ],\n    \"name\": \"ilks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"duration\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"initialOffer\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"dust\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ladle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ILadle\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"lockRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes12\",\n        \"name\": \"vaultId\",\n        \"type\": \"bytes12\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"min\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"payAll\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"ink\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"renounceRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4[]\",\n        \"name\": \"roles\",\n        \"type\": \"bytes4[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRoles\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes6\",\n        \"name\": \"ilkId\",\n        \"type\": \"bytes6\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"duration\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"initialOffer\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"dust\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"setIlk\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"role\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"adminRole\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"setRoleAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        },
    );
    #[derive(Clone)]
    pub struct Witch<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for Witch<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for Witch<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Witch))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> Witch<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), WITCH_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `LOCK` (0xa4f0d7d0) function"]
        pub fn lock(&self) -> ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([164, 240, 215, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT` (0x5909c12f) function"]
        pub fn root(&self) -> ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([89, 9, 193, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `auction` (0xb46b07f3) function"]
        pub fn auction(
            &self,
            vault_id: [u8; 12],
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 107, 7, 243], vault_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `auctions` (0xc6b13d5b) function"]
        pub fn auctions(
            &self,
            p0: [u8; 12],
        ) -> ethers_contract::builders::ContractCall<M, (ethers_core::types::Address, u32)>
        {
            self.0
                .method_hash([198, 177, 61, 91], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `buy` (0x7b141c5b) function"]
        pub fn buy(
            &self,
            vault_id: [u8; 12],
            base: u128,
            min: u128,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([123, 20, 28, 91], (vault_id, base, min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cauldron` (0x97ff6a04) function"]
        pub fn cauldron(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([151, 255, 106, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x5ba5e9f0) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 4],
        ) -> ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([91, 165, 233, 240], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0xde02cde7) function"]
        pub fn grant_role(
            &self,
            role: [u8; 4],
            account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 2, 205, 231], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRoles` (0xeffae353) function"]
        pub fn grant_roles(
            &self,
            roles: ::std::vec::Vec<[u8; 4]>,
            account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 250, 227, 83], (roles, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x10ab9432) function"]
        pub fn has_role(
            &self,
            role: [u8; 4],
            account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([16, 171, 148, 50], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ilks` (0x76c62635) function"]
        pub fn ilks(
            &self,
            p0: [u8; 6],
        ) -> ethers_contract::builders::ContractCall<M, (u32, u64, u128)> {
            self.0
                .method_hash([118, 198, 38, 53], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ladle` (0x94b8e38f) function"]
        pub fn ladle(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([148, 184, 227, 143], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lockRole` (0x559742d9) function"]
        pub fn lock_role(&self, role: [u8; 4]) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 151, 66, 217], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payAll` (0xf837dc72) function"]
        pub fn pay_all(
            &self,
            vault_id: [u8; 12],
            min: u128,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([248, 55, 220, 114], (vault_id, min))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x687f0e4c) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 4],
            account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 127, 14, 76], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0x44faded0) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 4],
            account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 250, 222, 208], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRoles` (0xad82110f) function"]
        pub fn revoke_roles(
            &self,
            roles: ::std::vec::Vec<[u8; 4]>,
            account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 130, 17, 15], (roles, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIlk` (0x81b20b92) function"]
        pub fn set_ilk(
            &self,
            ilk_id: [u8; 6],
            duration: u32,
            initial_offer: u64,
            dust: u128,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 178, 11, 146], (ilk_id, duration, initial_offer, dust))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRoleAdmin` (0xae93c1b5) function"]
        pub fn set_role_admin(
            &self,
            role: [u8; 4],
            admin_role: [u8; 4],
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 147, 193, 181], (role, admin_role))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Auctioned` event"]
        pub fn auctioned_filter(&self) -> ethers_contract::builders::Event<M, AuctionedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Bought` event"]
        pub fn bought_filter(&self) -> ethers_contract::builders::Event<M, BoughtFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IlkSet` event"]
        pub fn ilk_set_filter(&self) -> ethers_contract::builders::Event<M, IlkSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, WitchEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Auctioned", abi = "Auctioned(bytes12,uint256)")]
    pub struct AuctionedFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub start: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Bought", abi = "Bought(bytes12,address,uint256,uint256)")]
    pub struct BoughtFilter {
        #[ethevent(indexed)]
        pub vault_id: [u8; 12],
        #[ethevent(indexed)]
        pub buyer: ethers_core::types::Address,
        pub ink: ethers_core::types::U256,
        pub art: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "IlkSet", abi = "IlkSet(bytes6,uint32,uint64,uint128)")]
    pub struct IlkSetFilter {
        #[ethevent(indexed)]
        pub ilk_id: [u8; 6],
        pub duration: u32,
        pub initial_offer: u64,
        pub dust: u128,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "RoleAdminChanged", abi = "RoleAdminChanged(bytes4,bytes4)")]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 4],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 4],
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes4,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 4],
        #[ethevent(indexed)]
        pub account: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes4,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 4],
        #[ethevent(indexed)]
        pub account: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum WitchEvents {
        AuctionedFilter(AuctionedFilter),
        BoughtFilter(BoughtFilter),
        IlkSetFilter(IlkSetFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ethers_core::abi::Tokenizable for WitchEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuctionedFilter::from_token(token.clone()) {
                return Ok(WitchEvents::AuctionedFilter(decoded));
            }
            if let Ok(decoded) = BoughtFilter::from_token(token.clone()) {
                return Ok(WitchEvents::BoughtFilter(decoded));
            }
            if let Ok(decoded) = IlkSetFilter::from_token(token.clone()) {
                return Ok(WitchEvents::IlkSetFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::from_token(token.clone()) {
                return Ok(WitchEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::from_token(token.clone()) {
                return Ok(WitchEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::from_token(token.clone()) {
                return Ok(WitchEvents::RoleRevokedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                WitchEvents::AuctionedFilter(element) => element.into_token(),
                WitchEvents::BoughtFilter(element) => element.into_token(),
                WitchEvents::IlkSetFilter(element) => element.into_token(),
                WitchEvents::RoleAdminChangedFilter(element) => element.into_token(),
                WitchEvents::RoleGrantedFilter(element) => element.into_token(),
                WitchEvents::RoleRevokedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for WitchEvents {}
    impl ethers_contract::EthLogDecode for WitchEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuctionedFilter::decode_log(log) {
                return Ok(WitchEvents::AuctionedFilter(decoded));
            }
            if let Ok(decoded) = BoughtFilter::decode_log(log) {
                return Ok(WitchEvents::BoughtFilter(decoded));
            }
            if let Ok(decoded) = IlkSetFilter::decode_log(log) {
                return Ok(WitchEvents::IlkSetFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(WitchEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(WitchEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(WitchEvents::RoleRevokedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
