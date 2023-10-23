use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub meta_title: String,
    pub meta_owner: String,
    pub meta_description: Vec<MetaDesc>,
    pub meta_image: Vec<MetaImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Raw {
    pub owner: Vec<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub raw: Vec<Raw>,
    pub meta: Vec<Meta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub prop_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaDesc {
    pub prop_type: String,
    pub prop_field: String,
    pub prop_default: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaImage {
    pub prop_type: String,
    pub prop_field: String,
    pub prop_default: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    pub kind: String,
    pub networkId: u64,
    pub from: String,
    pub fromHistorical: String,
    pub readAbiFrom: String,
    pub startBlock: u64,
    pub endBlock: u64,
    pub interestedEvents: Vec<String>,
    pub interestedMethods: Vec<String>,
    pub handlersEvents: Vec<String>,
    pub handlersMethods: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct features {
    pub vectorize: bool,
    pub searchable: bool,
    pub querable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamType {
    pub param_name: String,
    pub param_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub meta_id: String,
    pub ipfs: String,
    pub meta_ownership: Vec<MetaOwnership>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaOwnership {
    pub kind: String,
    pub field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxnSource {
    pub method: String,
    pub action_type: String,
    pub meta_slug: String,
    pub param_types: Vec<ParamType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub slug: String,
    pub name: String,
    pub image: String,
    pub description: String,
    pub source: Vec<Source>,
    pub features: Vec<features>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub env: String,
    pub slug: String,
    pub mode: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Db {
    pub client: String,
    pub database: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbEnv {
    pub prod: Db,
    pub dev: Db,
}
