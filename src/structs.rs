use ethers::{
    abi::Token,
    types::{H160, H256, U256, U64},
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct MethodParam<'a> {
    pub name: &'a String,
    pub kind: String,
    pub internal_type: &'a std::option::Option<std::string::String>,
    pub value: Token,
}

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
    pub block_number: Option<U64>,
    pub contract_slug: String,
    pub contract_address: String,
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

pub struct MetaSchema {
    pub slug: String,
    pub contract_slug: String,
    pub data: Vec<String>
}