use ethers::{
    abi::Token,
    types::{H160, H256, U256, U64},
};
use std::{collections::HashMap, any::Any};
use mongodb::bson::{Document, document::ValueAccessError};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone,Deserialize)]
pub enum MethodParamDataType {
    StringValue,
    ComplexData
}

#[derive(Serialize, Debug, Clone)]
pub enum MethodParamValue {
    StringValue(String),
    ComplexData(HashMap<String, String>)
}

#[derive(Serialize, Debug, Clone,Deserialize)]
pub struct MethodParam {
    pub name: String,
    pub kind: String,
    pub internal_type: std::option::Option<std::string::String>,
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

#[derive(Serialize, Clone,Debug,Deserialize)]
pub struct TransactionData {
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
    pub method_params: Vec<MethodParam>,
    // pub status: Option<U64>
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

#[derive(Serialize, Clone)]
pub struct NetworkMetaData {
    pub network_name: String,
    pub network_rpc_url: String,
    pub start_block_number: i64,
    pub network_api_key: String
}

pub struct MetaSchema {
    pub slug: String,
    pub contract_slug: String,
    pub data: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaSchemaAbstractor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub slug: String,
    pub data: Vec<DataSchema>,
    pub source: Vec<SourceSchema>,
    pub reference: ReferenceSchema
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DataSchema {
    pub prop: String,
    pub prop_type: String,
    #[serde(rename = "prop_field", skip_serializing_if = "Option::is_none")]
    pub prop_field: Option<String>,
    #[serde(rename = "prop_default", skip_serializing_if = "Option::is_none")]
    pub prop_default: Option<serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceSchema {
    pub contract: String,
    pub action_type: String,
    pub method: String,
    pub data: Vec<DataSchema>,
    pub last_block_number: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceSchema {
    #[serde(rename = "metaId", skip_serializing_if = "Option::is_none")]
    pub metaId: Option<String>,
    #[serde(rename = "ipfs", skip_serializing_if = "Option::is_none")]
    pub ipfs: Option<String>,
    #[serde(rename = "erc721_module", skip_serializing_if = "Option::is_none")]
    pub erc721_module: Option<String>
}


#[derive(Serialize, Debug)]
pub struct Meta {
    pub slug: String,
    pub data: HashMap<String, HashMap<String, serde_json::Value>>,
    pub sources: Vec<MetaSource>,
    pub indexable: bool
}

#[derive(Serialize, Debug)]
pub struct MetaSource {
    pub contract: String,
    pub method: String,
    pub action_type: String,
    pub value: String // this is the transaction hash
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct IndexedTransaction{
    pub timestamp: String,
    pub transaction: TransactionData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionSchema {
    pub block_hash: serde_json::Value,
    pub block_number: serde_json::Value,
    pub contract_slug: serde_json::Value,
    pub contract_address: serde_json::Value,
    pub gas_used: serde_json::Value,
    pub gas_price: serde_json::Value,
    pub from: String,
    pub to: serde_json::Value,
    pub txn_hash: String,
    pub method_name: serde_json::Value,
    pub method_id: serde_json::Value,
    pub method_params: Vec<MethodParamAbstractor>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodParamAbstractor {
    pub name: String,
    pub kind: String,
    pub internal_type: std::option::Option<String>,
    pub value: serde_json::Value,
}