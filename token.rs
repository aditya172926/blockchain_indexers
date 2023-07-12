pub use erc20_token::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod erc20_token {
    const _: () = {
        ::core::include_bytes!("D:\\VSCode Rust\\indexing-entity\\abi\\abi_1.json");
    };
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract ENS\",\"name\":\"_old\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewOwner\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewResolver\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint64\",\"name\":\"ttl\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTTL\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"old\",\"outputs\":[{\"internalType\":\"contract ENS\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"recordExists\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"resolver\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"ttl\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRecord\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setResolver\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSubnodeOwner\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"ttl\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSubnodeRecord\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"ttl\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTTL\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"node\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ttl\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ERC20TOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ERC20Token<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC20Token<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC20Token<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC20Token<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC20Token<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ERC20Token)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC20Token<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20TOKEN_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `old` (0xb83f8663) function
        pub fn old(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([184, 63, 134, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x02571be3) function
        pub fn owner(
            &self,
            node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([2, 87, 27, 227], node)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordExists` (0xf79fe538) function
        pub fn record_exists(
            &self,
            node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([247, 159, 229, 56], node)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resolver` (0x0178b8bf) function
        pub fn resolver(
            &self,
            node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([1, 120, 184, 191], node)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x5b0fc9c3) function
        pub fn set_owner(
            &self,
            node: [u8; 32],
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 15, 201, 195], (node, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRecord` (0xcf408823) function
        pub fn set_record(
            &self,
            node: [u8; 32],
            owner: ::ethers::core::types::Address,
            resolver: ::ethers::core::types::Address,
            ttl: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 64, 136, 35], (node, owner, resolver, ttl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setResolver` (0x1896f70a) function
        pub fn set_resolver(
            &self,
            node: [u8; 32],
            resolver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 150, 247, 10], (node, resolver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSubnodeOwner` (0x06ab5923) function
        pub fn set_subnode_owner(
            &self,
            node: [u8; 32],
            label: [u8; 32],
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([6, 171, 89, 35], (node, label, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSubnodeRecord` (0x5ef2c7f0) function
        pub fn set_subnode_record(
            &self,
            node: [u8; 32],
            label: [u8; 32],
            owner: ::ethers::core::types::Address,
            resolver: ::ethers::core::types::Address,
            ttl: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 242, 199, 240], (node, label, owner, resolver, ttl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTTL` (0x14ab9038) function
        pub fn set_ttl(
            &self,
            node: [u8; 32],
            ttl: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 171, 144, 56], (node, ttl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ttl` (0x16a25cbd) function
        pub fn ttl(
            &self,
            node: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([22, 162, 92, 189], node)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewOwner` event
        pub fn new_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewResolver` event
        pub fn new_resolver_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewResolverFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewTTL` event
        pub fn new_ttl_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewTTLFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ERC20TokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC20Token<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "NewOwner", abi = "NewOwner(bytes32,bytes32,address)")]
    pub struct NewOwnerFilter {
        #[ethevent(indexed)]
        pub node: [u8; 32],
        #[ethevent(indexed)]
        pub label: [u8; 32],
        pub owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "NewResolver", abi = "NewResolver(bytes32,address)")]
    pub struct NewResolverFilter {
        #[ethevent(indexed)]
        pub node: [u8; 32],
        pub resolver: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "NewTTL", abi = "NewTTL(bytes32,uint64)")]
    pub struct NewTTLFilter {
        #[ethevent(indexed)]
        pub node: [u8; 32],
        pub ttl: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(bytes32,address)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub node: [u8; 32],
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ERC20TokenEvents {
        ApprovalForAllFilter(ApprovalForAllFilter),
        NewOwnerFilter(NewOwnerFilter),
        NewResolverFilter(NewResolverFilter),
        NewTTLFilter(NewTTLFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC20TokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC20TokenEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = NewOwnerFilter::decode_log(log) {
                return Ok(ERC20TokenEvents::NewOwnerFilter(decoded));
            }
            if let Ok(decoded) = NewResolverFilter::decode_log(log) {
                return Ok(ERC20TokenEvents::NewResolverFilter(decoded));
            }
            if let Ok(decoded) = NewTTLFilter::decode_log(log) {
                return Ok(ERC20TokenEvents::NewTTLFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20TokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC20TokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewOwnerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewResolverFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewTTLFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC20TokenEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<NewOwnerFilter> for ERC20TokenEvents {
        fn from(value: NewOwnerFilter) -> Self {
            Self::NewOwnerFilter(value)
        }
    }
    impl ::core::convert::From<NewResolverFilter> for ERC20TokenEvents {
        fn from(value: NewResolverFilter) -> Self {
            Self::NewResolverFilter(value)
        }
    }
    impl ::core::convert::From<NewTTLFilter> for ERC20TokenEvents {
        fn from(value: NewTTLFilter) -> Self {
            Self::NewTTLFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC20TokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `old` function with signature `old()` and selector `0xb83f8663`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "old", abi = "old()")]
    pub struct OldCall;
    ///Container type for all input parameters for the `owner` function with signature `owner(bytes32)` and selector `0x02571be3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner(bytes32)")]
    pub struct OwnerCall {
        pub node: [u8; 32],
    }
    ///Container type for all input parameters for the `recordExists` function with signature `recordExists(bytes32)` and selector `0xf79fe538`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recordExists", abi = "recordExists(bytes32)")]
    pub struct RecordExistsCall {
        pub node: [u8; 32],
    }
    ///Container type for all input parameters for the `resolver` function with signature `resolver(bytes32)` and selector `0x0178b8bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "resolver", abi = "resolver(bytes32)")]
    pub struct ResolverCall {
        pub node: [u8; 32],
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setOwner` function with signature `setOwner(bytes32,address)` and selector `0x5b0fc9c3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(bytes32,address)")]
    pub struct SetOwnerCall {
        pub node: [u8; 32],
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRecord` function with signature `setRecord(bytes32,address,address,uint64)` and selector `0xcf408823`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setRecord", abi = "setRecord(bytes32,address,address,uint64)")]
    pub struct SetRecordCall {
        pub node: [u8; 32],
        pub owner: ::ethers::core::types::Address,
        pub resolver: ::ethers::core::types::Address,
        pub ttl: u64,
    }
    ///Container type for all input parameters for the `setResolver` function with signature `setResolver(bytes32,address)` and selector `0x1896f70a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setResolver", abi = "setResolver(bytes32,address)")]
    pub struct SetResolverCall {
        pub node: [u8; 32],
        pub resolver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setSubnodeOwner` function with signature `setSubnodeOwner(bytes32,bytes32,address)` and selector `0x06ab5923`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setSubnodeOwner",
        abi = "setSubnodeOwner(bytes32,bytes32,address)"
    )]
    pub struct SetSubnodeOwnerCall {
        pub node: [u8; 32],
        pub label: [u8; 32],
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setSubnodeRecord` function with signature `setSubnodeRecord(bytes32,bytes32,address,address,uint64)` and selector `0x5ef2c7f0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setSubnodeRecord",
        abi = "setSubnodeRecord(bytes32,bytes32,address,address,uint64)"
    )]
    pub struct SetSubnodeRecordCall {
        pub node: [u8; 32],
        pub label: [u8; 32],
        pub owner: ::ethers::core::types::Address,
        pub resolver: ::ethers::core::types::Address,
        pub ttl: u64,
    }
    ///Container type for all input parameters for the `setTTL` function with signature `setTTL(bytes32,uint64)` and selector `0x14ab9038`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTTL", abi = "setTTL(bytes32,uint64)")]
    pub struct SetTTLCall {
        pub node: [u8; 32],
        pub ttl: u64,
    }
    ///Container type for all input parameters for the `ttl` function with signature `ttl(bytes32)` and selector `0x16a25cbd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ttl", abi = "ttl(bytes32)")]
    pub struct TtlCall {
        pub node: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ERC20TokenCalls {
        IsApprovedForAll(IsApprovedForAllCall),
        Old(OldCall),
        Owner(OwnerCall),
        RecordExists(RecordExistsCall),
        Resolver(ResolverCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetOwner(SetOwnerCall),
        SetRecord(SetRecordCall),
        SetResolver(SetResolverCall),
        SetSubnodeOwner(SetSubnodeOwnerCall),
        SetSubnodeRecord(SetSubnodeRecordCall),
        SetTTL(SetTTLCall),
        Ttl(TtlCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC20TokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded)
                = <OldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Old(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RecordExistsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecordExists(decoded));
            }
            if let Ok(decoded)
                = <ResolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Resolver(decoded));
            }
            if let Ok(decoded)
                = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded)
                = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOwner(decoded));
            }
            if let Ok(decoded)
                = <SetRecordCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRecord(decoded));
            }
            if let Ok(decoded)
                = <SetResolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetResolver(decoded));
            }
            if let Ok(decoded)
                = <SetSubnodeOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSubnodeOwner(decoded));
            }
            if let Ok(decoded)
                = <SetSubnodeRecordCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetSubnodeRecord(decoded));
            }
            if let Ok(decoded)
                = <SetTTLCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetTTL(decoded));
            }
            if let Ok(decoded)
                = <TtlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ttl(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC20TokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Old(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRecord(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetResolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSubnodeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSubnodeRecord(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTTL(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ttl(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC20TokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Old(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRecord(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetResolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSubnodeOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSubnodeRecord(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTTL(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ttl(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ERC20TokenCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<OldCall> for ERC20TokenCalls {
        fn from(value: OldCall) -> Self {
            Self::Old(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ERC20TokenCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RecordExistsCall> for ERC20TokenCalls {
        fn from(value: RecordExistsCall) -> Self {
            Self::RecordExists(value)
        }
    }
    impl ::core::convert::From<ResolverCall> for ERC20TokenCalls {
        fn from(value: ResolverCall) -> Self {
            Self::Resolver(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ERC20TokenCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for ERC20TokenCalls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    impl ::core::convert::From<SetRecordCall> for ERC20TokenCalls {
        fn from(value: SetRecordCall) -> Self {
            Self::SetRecord(value)
        }
    }
    impl ::core::convert::From<SetResolverCall> for ERC20TokenCalls {
        fn from(value: SetResolverCall) -> Self {
            Self::SetResolver(value)
        }
    }
    impl ::core::convert::From<SetSubnodeOwnerCall> for ERC20TokenCalls {
        fn from(value: SetSubnodeOwnerCall) -> Self {
            Self::SetSubnodeOwner(value)
        }
    }
    impl ::core::convert::From<SetSubnodeRecordCall> for ERC20TokenCalls {
        fn from(value: SetSubnodeRecordCall) -> Self {
            Self::SetSubnodeRecord(value)
        }
    }
    impl ::core::convert::From<SetTTLCall> for ERC20TokenCalls {
        fn from(value: SetTTLCall) -> Self {
            Self::SetTTL(value)
        }
    }
    impl ::core::convert::From<TtlCall> for ERC20TokenCalls {
        fn from(value: TtlCall) -> Self {
            Self::Ttl(value)
        }
    }
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `old` function with signature `old()` and selector `0xb83f8663`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OldReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner(bytes32)` and selector `0x02571be3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `recordExists` function with signature `recordExists(bytes32)` and selector `0xf79fe538`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RecordExistsReturn(pub bool);
    ///Container type for all return fields from the `resolver` function with signature `resolver(bytes32)` and selector `0x0178b8bf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ResolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setSubnodeOwner` function with signature `setSubnodeOwner(bytes32,bytes32,address)` and selector `0x06ab5923`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SetSubnodeOwnerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ttl` function with signature `ttl(bytes32)` and selector `0x16a25cbd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TtlReturn(pub u64);
}
