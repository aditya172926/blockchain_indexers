use ethers::contract::{Contract, ContractInstance};
use ethers::core::abi::Abi;
use ethers::providers::{Http, Provider};
use ethers::types::{H160, H256};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ContractData {
    pub address: String,
    pub chain_id: String,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub image: String,
    pub interested_methods: Vec<String>,
    pub interested_events: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ContractEvent {
    pub topic0: H256,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct ContractEventMap {
    pub topics: Vec<H256>,
    pub map: HashMap<H256, String>,
    pub events: Vec<ContractEvent>,
}

#[derive(Clone, Debug)]
pub struct ContractMetaData {
    pub contract_address: String,
    pub contract_address_H160: H160,
    pub contract_address_historical: String,
    pub contract_address_historical_H160: H160,
    pub read_abi_from: String,
    pub read_abi_from_H160: H160,
    pub chain_id: u64,
    pub method_of_interest: Vec<std::string::String>,
    pub events_of_interest: ContractEventMap,
}

#[derive(Debug, Clone)]
pub struct ContractAbi {
    pub string: String,
    pub raw: web3::ethabi::Contract,
    pub stat: Abi,
}

#[derive(Clone, Debug)]
pub struct ContractIndexed {
    pub data: ContractMetaData,
    pub abi: ContractAbi,
    pub instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
}
