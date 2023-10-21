use serde::Serialize;

#[derive(Serialize, Clone,Debug)]
pub struct NetworkStruct {
    pub network_id: String,
    pub network_name: String,
    pub network_rpc_url: String,
    pub start_block_number: i64,
    pub network_api_key: String
}