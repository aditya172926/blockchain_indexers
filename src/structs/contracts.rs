
use std::{collections::HashMap, any::Any};
use ethers::abi::JsonAbi;
use ethers::types::H160;
use mongodb::bson::{Document, document::ValueAccessError};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use ethers::abi::Abi;




#[derive(Serialize)]
pub struct ContractData {
    pub address: String,
    pub chain_id: String,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub image: String,
    pub interested_methods: Vec<String>,
    pub interested_events: Vec<String>
}


#[derive( Clone, Debug)]
pub struct ContractMetaData {
    pub contract_address: String,
    
    pub read_abi_from: String,
    pub chain_id: String,
    pub function_of_interest: String,
    pub contract_name: String,
    pub contract_description: String,
    pub contract_slug: String,
    pub method_of_interest:std::collections::HashSet<String>,
    pub methods:Document,
}

#[derive(Debug)]
pub struct ContractAbi {
    pub string: String,
    pub raw: web3::ethabi::Contract,
    pub stat:&'static Abi,
}