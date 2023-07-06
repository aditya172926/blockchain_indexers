use std::fs;
use std::string::String;
use ethers::providers::Provider;

pub async fn get_network_rpc(network_id: &str) -> String {
    let network_details: String = fs::read_to_string(r"config/network.json").expect("Error");
    let network_details = serde_json::from_str::<serde_json::Value>(&network_details);
    let network_rpc = match network_details {
        Ok(object) => object[network_id]["networkRpcUrl"].to_string(),
        Err(e) => e.to_string(),
    };
    let network_rpc: String = network_rpc[1..network_rpc.len() - 1].to_string();
    println!("The Network RPC Endpoint is {:?}", network_rpc);
    return network_rpc;
}

// pub fn get_contract_metadata() {
//     let contract_meta_data: String = fs::read_to_string(r"config/global.json")?.parse()?;

// }

// pub async fn get_contract_instance(network_rpc: String) {
//     let provider = Provider::try_from(network_rpc);

// }