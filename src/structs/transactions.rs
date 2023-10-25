use ethers::{
    abi::Token,
    types::{H160, H256, U256},
};
use serde::{Deserialize, Serialize, Deserializer};

use super::index::MethodParam;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct TransactionMethod {
    pub name: String,
    pub id: String,
    pub params: Vec<Token>,
}
#[derive(Clone, Serialize, Debug,Deserialize)]
pub struct Transaction {
    pub block_hash: Option<H256>,
    pub block_number: u64,
    pub contract_address: String,
    pub chain_id: u64,
    pub gas_used: Option<U256>,
    pub gas_price: Option<U256>,
    pub from: H160,
    pub to: Option<H160>,
    pub txn_hash: H256,
    // pub status: Option<U64>
}

#[derive(Clone, Debug, Serialize)]
pub struct TransactionIndexed {
    pub timestamp: String,
    pub transaction: Transaction,
    pub method: TransactionMethod,
}