use std::fs;
use std::string::String;
use std::{error::Error, str::FromStr};


#[tokio::main]
pub async fn get_network_rpc(network_id: &str) -> Result<(), Box<dyn Error>> {
    let networkDetails = fs::read_to_string(r"config/network.json").unwrap().parse();
    let networkDetails = serde_json::from_str::<serde_json::Value>(&networkDetails);
    let networkEndpoint = match networkDetails {
        Ok(object) => object[network_id]["networkRpcUrl"].to_string(),
        Err(e) => e.to_string(),
    };
    // println!("{:?}", &networkEndpoint[1..networkEndpoint.len() - 1]);

    // println!("{:?}",networkDetails);
    // let networkEndpoint: String = networkDetails
    return &networkEndpoint[1..networkEndpoint.len() - 1];
}