use std::collections::HashMap;

use ethers::{
    abi::Token,
    types::{H160, H256, U256},
};
use serde::{Deserialize, Deserializer, Serialize};

use super::index::MethodParam;

#[derive(Serialize, Clone, Debug, Deserialize, Default)]
pub struct TransactionMethod {
    pub name: String,
    pub id: String,
    pub params: Vec<Token>,
}

#[derive(Serialize, Clone, Debug, Deserialize, Default)]
pub struct TransactionEvent {
    pub topic0: H256,
    pub name: String,
    pub params: Vec<Token>,
    pub data: Option<HashMap<String, String>>,
}

#[derive(Clone, Serialize, Debug, Deserialize, Default)]
pub struct Transaction {
    pub block_hash: Option<H256>,
    pub block_number: Option<u64>,
    pub contract_address: Option<String>,
    pub chain_id: Option<u64>,
    pub gas_used: Option<U256>,
    pub gas_price: Option<U256>,
    pub from: Option<H160>,
    pub to: Option<H160>,
    pub txn_hash: Option<String>,
    // pub status: Option<U64>
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct TransactionIndexed {
    pub timestamp: Option<String>,
    pub transaction: Transaction,
    pub method: Option<TransactionMethod>,
    pub events: Option<Vec<TransactionEvent>>,
}
