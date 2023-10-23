

// pub mod networks{
use std::fs;
use std::string::String;
use crate::structs::networks::NetworkStruct;


pub fn utils_network_data(chain_id: &str) -> Option<NetworkStruct> {
    let network_details: String =
        fs::read_to_string(r"config/network.json").expect("Error in reading network.json file");
    let network_details: Result<serde_json::Value, serde_json::Error> = serde_json::from_str::<serde_json::Value>(&network_details);

    let network_rpc = match network_details {
        Ok(object) => {
            let mut network_name: String = object["production"][chain_id]["network_name"].to_string();
            let mut network_rpc_url: String = object["production"][chain_id]["network_rpc_url"].to_string();
            let mut network_api_key: String = object["production"][chain_id]["network_api_key"].to_string();
            let start_block_number: i64 = object["production"][chain_id]["start_block_number"]
                .to_string()
                .parse()
                .unwrap();

            network_name = network_name[1..network_name.len() - 1].to_string();
            network_rpc_url = network_rpc_url[1..network_rpc_url.len() - 1].to_string();
            network_api_key = network_api_key[1..network_api_key.len() - 1].to_string();

            let result: NetworkStruct = NetworkStruct {
                network_id: chain_id.to_string(),
                network_name: network_name,
                network_rpc_url: network_rpc_url,
                start_block_number: start_block_number,
                network_api_key: network_api_key
            };
            Some(result)
        }
        Err(e) => {
            println!("Error in getting network data {:?}", e);
            None
        }
    };
    println!("network metadata \n network = {:?}", network_rpc);
    return network_rpc;
}
// }
