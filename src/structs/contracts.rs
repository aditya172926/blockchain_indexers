use ethers::abi::Abi;
use ethers::abi::JsonAbi;
use ethers::types::H160;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{document::ValueAccessError, Document};
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap};

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
pub struct ContractMetaData {
    pub contract_address: String,
    pub contract_address_historical: String,
    pub read_abi_from: String,
    pub chain_id: u64,
    pub start_block:u64,
    pub end_block:u64,
    pub method_of_interest: Vec<std::string::String>,
}

#[derive(Debug)]
pub struct ContractAbi {
    pub string: String,
    pub raw: web3::ethabi::Contract,
    pub stat: &'static Abi,
}
