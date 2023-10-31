use ethers::core::abi::Abi;
use ethers::types::{H160, H256};
use serde::Serialize;

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
pub struct ContractMetaData {
    pub contract_address: String,
    pub contract_address_H160: H160,
    pub contract_address_historical: String,
    pub contract_address_historical_H160: H160,
    pub read_abi_from: String,
    pub read_abi_from_H160: H160,
    pub chain_id: u64,
    pub method_of_interest: Vec<std::string::String>,
    pub events_of_interest: Vec<ContractEvent>
}

#[derive(Debug)]
pub struct ContractAbi {
    pub string: String,
    pub raw: web3::ethabi::Contract,
    pub stat: Abi,
}
