use ethers::providers::Provider;
use std::fmt;
use std::fs;
use std::string::String;
use url::ParseError;

pub fn get_network_rpc(network_id: &str) -> String {
    let network_details: String =
        fs::read_to_string(r"config/network.json").expect("Error in reading network.json file");
    let network_details = serde_json::from_str::<serde_json::Value>(&network_details);
    let network_rpc = match network_details {
        Ok(object) => object[network_id]["networkRpcUrl"].to_string(),
        Err(e) => e.to_string(),
    };
    let network_rpc: String = network_rpc[1..network_rpc.len() - 1].to_string();
    println!("The Network RPC Endpoint is {:?}", network_rpc);
    return network_rpc;
}

pub fn get_contract_metadata(protocol_name: &str) -> String {
    let contract_meta_data: String =
        fs::read_to_string(r"config/global.json").expect("Error in reading global.json file");
    let contract_meta_data = serde_json::from_str::<serde_json::Value>(&contract_meta_data);
    let contract_chain_id: String;
    let mut contract_address: String = "".to_string();
    match contract_meta_data {
        Ok(object) => {
            contract_address = object[protocol_name]["contract_address"].to_string();
            contract_chain_id = object[protocol_name]["chainId"].to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
    contract_address = contract_address[1..contract_address.len() - 1].to_string();
    return contract_address;
}

// pub async fn get_provider(
//     chain_id: String,
// ) -> std::result::Result<Provider<ethers::providers::Http>,  ethers::abi::ParseError> {
//     let network_data: String =
//         fs::read_to_string(r"config/network.json").expect("Error in reading network.json file");
//     let network_data = serde_json::from_str::<serde_json::Value>(&network_data);
//     let mut network_rpc: String = String::new();
//     match network_data {
//         Ok(object) => {
//             network_rpc = object[chain_id]["networkRpcUrl"].to_string();
//             let provider = Provider::try_from(network_rpc);
//             println!("The provider is {:?}", provider);
//             return provider;
//         }
//         Err(e) => {
//             println!("Error in reading networkRpcUrl {:?}", e);
//             return ethers::abi::ParseError::ParseError(e);
//         }
//     }
// }

pub async fn fetch_contract_abi(
    network_name: String,
    contract_address: &str,
) -> reqwest::Result<reqwest::Response> {
    let file: String = fs::read_to_string(r"config/constants.json")
        .expect("Error in reading the constants.json file");
    let file_data = serde_json::from_str::<serde_json::Value>(&file);

    let mut api: String = String::new();
    match file_data {
        Ok(object) => {
            api = object[network_name]["_api"].to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let mut api_url = str::replace(&api, "{}", &contract_address);
    api_url = api_url[1..api_url.len() - 1].to_string();
    println!("The api_url is {}", api_url);

    let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(&api_url).await;
    // let mut fetched_abi: reqwest::Response = Default::default();
    return response;
}
