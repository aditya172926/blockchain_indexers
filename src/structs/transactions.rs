#[derive(Serialize, Clone,Debug,Deserialize)]
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
    pub method_name: String,
    pub method_id: String,
    pub method_params: Vec<MethodParam>,
    // pub status: Option<U64>
}



#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct TransactionIndexed{
    pub timestamp: String,
    pub transaction: Transaction
}