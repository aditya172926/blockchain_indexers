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
    pub chain_id: String,
    pub function_of_interest: String,
    pub contract_name: String,
    pub contract_description: String,
    pub contract_slug: String,
    pub method_of_interest: std::collections::HashSet<String>,
    pub methods: Document,
}

#[derive(Debug)]
pub struct ContractAbi {
    pub string: String,
    pub raw: web3::ethabi::Contract,
    pub stat: &'static Abi,
}
