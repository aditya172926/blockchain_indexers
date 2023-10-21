
use ethers::types::{U256, H256, H160};
use serde::{Serialize, Deserialize};

use super::index::MethodParam;

#[derive(Serialize, Clone,Debug,Deserialize)]

pub struct TransactionMethod {
    pub name: String,
    pub id: String,
    pub params: Vec<MethodParam>,
}
#[derive(Clone,Serialize,Debug)]
pub struct Transaction {
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
    // pub status: Option<U64>
}



#[derive(Clone,Debug)]
pub struct TransactionIndexed{
    pub timestamp: String,
    pub transaction: Transaction,
    pub method : TransactionMethod,
}