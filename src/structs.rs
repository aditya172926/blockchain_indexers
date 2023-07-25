use ethers::{
    abi::Token,
    types::{H160, H256, U256, U64},
};
use std::{collections::HashMap, any::Any};
use mongodb::bson::{Document, document::ValueAccessError};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum MethodParamDataType {
    StringValue,
    ComplexData
}

#[derive(Serialize, Debug, Clone)]
pub enum MethodParamValue {
    StringValue(String),
    ComplexData(HashMap<String, String>)
}

#[derive(Serialize, Debug, Clone)]
pub struct MethodParam<'a> {
    pub name: String,
    pub kind: String,
    pub internal_type: &'a std::option::Option<std::string::String>,
    pub data_type: MethodParamDataType,
    pub value: String,
}

// #[derive(Serialize, Debug, Clone)]
// pub struct MethodParamList<'a> {
//     pub name: &'a String,
//     pub kind: String,
//     pub internal_type: &'a std::option::Option<std::string::String>,
//     pub value: HashMap<String, String>,
// }

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

#[derive(Serialize, Clone)]
pub struct TransactionData <'a>{
    pub block_hash: Option<H256>,
    pub block_number: u64,
    pub contract_slug: String,
    pub contract_address: String,
    pub chain_id: String,
    pub gas_used: Option<U256>,
    pub gas_price: Option<U256>,
    pub from: H160,
    pub to: Option<H160>,
    pub txn_hash: H256,
    pub method_name: String,
    pub method_id: String,
    pub method_params: Vec<MethodParam<'a>>,
    // pub status: Option<U64>
}

#[derive( Clone, Debug)]
pub struct ContractMetaData {
    pub contract_address: ethcontract::H160,
    pub read_abi_from: String,
    pub chain_id: String,
    pub function_of_interest: String,
    pub contract_name: String,
    pub contract_description: String,
    pub contract_slug: String,
    pub method_of_interest:std::collections::HashSet<String>,
    pub methods:Document,
}

#[derive(Serialize, Clone)]
pub struct NetworkMetaData {
    pub network_name: String,
    pub network_rpc_url: String,
    pub start_block_number: i64
}

pub struct MetaSchema {
    pub slug: String,
    pub contract_slug: String,
    pub data: Vec<String>
}