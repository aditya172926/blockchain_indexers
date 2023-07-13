use std::fs;
use std::string::String;

pub fn get_network_rpc(chain_id: &str) -> String {
    let network_details: String =
        fs::read_to_string(r"config/network.json").expect("Error in reading network.json file");
    let network_details = serde_json::from_str::<serde_json::Value>(&network_details);
    let network_rpc = match network_details {
        Ok(object) => object[chain_id]["networkRpcUrl"].to_string(),
        Err(e) => e.to_string(),
    };
    let network_rpc: String = network_rpc[1..network_rpc.len() - 1].to_string();
    // println!("The Network RPC Endpoint is {:?}", network_rpc);
    return network_rpc;
}

pub fn get_contract_metadata(protocol_name: &str) -> (String, String, String, String, String, String) {
    let contract_meta_data: String =
        fs::read_to_string(r"config/global.json").expect("Error in reading global.json file");
    let contract_meta_data = serde_json::from_str::<serde_json::Value>(&contract_meta_data);
    let mut contract_chain_id: String = String::new();
    let mut contract_address: String = String::new();
    let mut function_of_interest: String = String::new();
    let mut contract_name: String = String::new();
    let mut contract_description: String = String::new();
    let mut contract_slug: String = String::new();
    match contract_meta_data {
        Ok(object) => {
            contract_address = object[protocol_name]["contract_address"].to_string();
            contract_chain_id = object[protocol_name]["chainId"].to_string();
            function_of_interest = object[protocol_name]["function_of_interest"].to_string();
            contract_name = object[protocol_name]["name"].to_string();
            contract_description = object[protocol_name]["description"].to_string();
            contract_slug = object[protocol_name]["slug"].to_string();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
    contract_chain_id = contract_chain_id[1..contract_chain_id.len() - 1].to_string();
    contract_address = contract_address[1..contract_address.len() - 1].to_string();
    contract_name = contract_name[1..contract_name.len()-1].to_string();
    contract_description = contract_description[1..contract_description.len()-1].to_string();
    contract_slug = contract_slug[1..contract_slug.len()-1].to_string();
    return (contract_address, contract_chain_id, function_of_interest, contract_name, contract_description, contract_slug);
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
    let response: Result<reqwest::Response, reqwest::Error> = fetch_contract_abi(contract_chain_id.to_string(), contract_address).await;
    // let contract_abi: Result<String, reqwest::Error>;
    let mut fetched_abi: String = String::new();

    match response {
        Ok(object) => {
            if object.status().is_success() {
                // Read the response body as a string
                let response_body: String = object.text().await.expect("Error in parsing object");
                // Parse the response body as JSON
                let json: serde_json::Value = serde_json::from_str(&response_body).expect("Error in reading to json format");
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