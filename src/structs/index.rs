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
    pub last_block_number: i64
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

pub struct MetaData {
    pub id: String,
    pub title: String,
    pub media: String,
    pub content: String,
    pub context: String,
    pub context: String,
    pub created_at: String,
    pub updated_at: String,
}






#[derive(Serialize, Deserialize, Debug)]
pub struct MethodParamAbstractor {
    pub name: String,
    pub kind: String,
    pub internal_type: std::option::Option<String>,
    pub value: serde_json::Value,
}