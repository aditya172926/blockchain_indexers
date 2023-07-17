use crate::structs::{ContractMetaData, NetworkMetaData};
use std::fs;
use std::string::String;
use std::collections::HashSet;

pub fn get_network_data(chain_id: &str) -> Option<NetworkMetaData> {
    let network_details: String =
        fs::read_to_string(r"config/network.json").expect("Error in reading network.json file");
    let network_details = serde_json::from_str::<serde_json::Value>(&network_details);

    let network_rpc = match network_details {
        Ok(object) => {
            let mut network_name: String = object[chain_id]["network_name"].to_string();
            let mut network_rpc_url: String = object[chain_id]["network_rpc_url"].to_string();
            let start_block_number: i64 = object[chain_id]["start_block_number"]
                .to_string()
                .parse()
                .unwrap();

            network_name = network_name[1..network_name.len() - 1].to_string();
            network_rpc_url = network_rpc_url[1..network_rpc_url.len() - 1].to_string();

            let result: NetworkMetaData = NetworkMetaData {
                network_name: network_name,
                network_rpc_url: network_rpc_url,
                start_block_number: start_block_number,
            };
            Some(result)
        }
        Err(e) => {
            println!("Error in getting network data {:?}", e);
            None
        }
    };
    return network_rpc;
}

pub fn get_contract_metadata(protocol_name: &str) -> Option<ContractMetaData> {
    let contract_meta_data: String =
        fs::read_to_string(r"config/global.json").expect("Error in reading global.json file");
    let contract_meta_data: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str::<serde_json::Value>(&contract_meta_data);

    let contract_metadata: Option<ContractMetaData> = match contract_meta_data {
        Ok(object) => {
            let mut contract_address: String =
                object[protocol_name]["contract_address"].to_string();
            let mut contract_chain_id: String = object[protocol_name]["chainId"].to_string();
            let mut function_of_interest: String =
                object[protocol_name]["function_of_interest"].to_string();
            let mut contract_name: String = object[protocol_name]["name"].to_string();
            let mut contract_description: String = object[protocol_name]["description"].to_string();
            let mut contract_slug: String = object[protocol_name]["slug"].to_string();
            let mut read_abi_from: String = object[protocol_name]["read_abi_from"].to_string();
            let mut method_of_interest:HashSet<String> = HashSet::new();
            let size=&object["lens_polygon"]["method_of_interest"].as_array().unwrap().len();
            for i in 0..*size {
                let interested:&String=
                &object["lens_polygon"]["method_of_interest"][i].to_string().parse().unwrap();
                let item=interested[1..interested.len()-1].to_string();
     
                method_of_interest.insert(item);
            }

            contract_chain_id = contract_chain_id[1..contract_chain_id.len() - 1].to_string();
            read_abi_from = read_abi_from[1..read_abi_from.len() - 1].to_string();
            contract_address = contract_address[1..contract_address.len() - 1].to_string();
            contract_name = contract_name[1..contract_name.len() - 1].to_string();
            contract_description =
                contract_description[1..contract_description.len() - 1].to_string();
            contract_slug = contract_slug[1..contract_slug.len() - 1].to_string();

            let result: ContractMetaData = ContractMetaData {
                contract_address: contract_address,
                read_abi_from: read_abi_from,
                chain_id: contract_chain_id,
                function_of_interest: function_of_interest,
                contract_name: contract_name,
                contract_description: contract_description,
                contract_slug: contract_slug,
                method_of_interest:method_of_interest
            };
            Some(result)
        }
        Err(e) => {
            println!("{:?}", e);
            None
        }
    };
    return contract_metadata;
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
    contract_chain_id: String,
    contract_address: &str,
) -> reqwest::Result<reqwest::Response> {
    // println!("The Chain id is {}", contract_chain_id);
    let file: String = fs::read_to_string(r"config/constants.json")
        .expect("Error in reading the constants.json file");
    let file_data = serde_json::from_str::<serde_json::Value>(&file);

    let mut api: String = String::new();
    match file_data {
        Ok(object) => {
            api = object[contract_chain_id]["_api"].to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let mut api_url = str::replace(&api, "{}", &contract_address);
    api_url = api_url[1..api_url.len() - 1].to_string();
    // println!("The api_url is {}", api_url);

    let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(&api_url).await;
    // let mut fetched_abi: reqwest::Response = Default::default();
    return response;
}

pub async fn format_contract_abi(contract_chain_id: &str, contract_address: &str) -> String {
    let response: Result<reqwest::Response, reqwest::Error> =
        fetch_contract_abi(contract_chain_id.to_string(), contract_address).await;
    // let contract_abi: Result<String, reqwest::Error>;
    let mut fetched_abi: String = String::new();

    match response {
        Ok(object) => {
            if object.status().is_success() {
                // Read the response body as a string
                let response_body: String = object.text().await.expect("Error in parsing object");
                // Parse the response body as JSON
                let json: serde_json::Value =
                    serde_json::from_str(&response_body).expect("Error in reading to json format");
                fetched_abi = json["result"].as_str().unwrap().to_owned();
                // println!("The fetched contract abi is {:?}", fetched_abi);
            } else {
                println!("Request failed with status code: {}", object.status());
            }
            return fetched_abi;
        }
        Err(e) => {
            println!("Error in fetching contract abi {:?}", e);
            return "Error in response".to_string();
        }
    }
}
