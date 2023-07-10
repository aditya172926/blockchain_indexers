use ethers::{
    abi::{ParamType, Token},
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

#[derive(Serialize, Clone)]
pub struct TransactionData <'a>{
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub contract_address: Option<H160>,
    pub gas_used: Option<U256>,
    pub gas_price: Option<U256>,
    pub from: H160,
    pub to: Option<H160>,
    pub txn_hash: H256,
    pub method_name: String,
    pub method_id: String,
    pub method_params: Vec<MethodParam<'a>>,
    pub status: Option<U64>
}
