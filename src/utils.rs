use crate::db;
use crate::structs::{ContractMetaData, NetworkMetaData, MethodParam};
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

pub async fn get_contract_data(
    protocol_name: &str,
) -> (ContractMetaData, String, web3::ethabi::Contract) {
    let contract_metadata: ContractMetaData = get_contract_metadata(protocol_name).await.unwrap();

    let mut contract_fetched_abi: String = String::new();
    if contract_metadata.read_abi_from.contains("0x") {
        contract_fetched_abi = format_contract_abi(
            &contract_metadata.chain_id,
            &contract_metadata.read_abi_from,
        )
        .await;
        contract_fetched_abi = format_contract_abi(
            &contract_metadata.chain_id,
            &contract_metadata.read_abi_from,
        )
        .await;
    } else {
        contract_fetched_abi = format_contract_abi(
            &contract_metadata.chain_id,
            &contract_metadata.contract_address.to_string(),
        )
        .await;
        contract_fetched_abi = format_contract_abi(
            &contract_metadata.chain_id,
            &contract_metadata.contract_address.to_string(),
        )
        .await;
    }

    let contract_abi: web3::ethabi::Contract = serde_json::from_str(&contract_fetched_abi).unwrap();

    return (contract_metadata, contract_fetched_abi, contract_abi);
}

pub async fn get_contract_metadata(protocol_name: &str) -> Option<ContractMetaData> {
    let contract_result: mongodb::bson::Document =
        db::db_contract_data(protocol_name).await.unwrap();
    let contract_meta_data: Result<
        &mongodb::bson::Document,
        mongodb::bson::document::ValueAccessError,
    > = contract_result.get_document("contract");

    let contract_metadata: Option<ContractMetaData> = match contract_meta_data {
        Ok(object) => {
            let mut contract_address_string: String =
                object.get_str("address").unwrap().to_string();
            let mut contract_chain_id: String = object.get_str("chain_id").unwrap().to_string();
            let mut function_of_interest: String = "".to_string();
            let mut contract_name: String = object.get_str("name").unwrap().to_string();
            let mut contract_description: String =
                object.get_str("description").unwrap().to_string();
            let mut contract_slug: String = object.get_str("slug").unwrap().to_string();
            let mut read_abi_result: Result<&str, mongodb::bson::document::ValueAccessError> = object.get_str("read_abi_from");
            let mut methods=object.get_document("methods").unwrap();
            // let interested=object.get_array("interested_methods").unwrap()[0].to_string();
            // let interested="post";
            // println!("{}",interested);
            // println!("{:?}",methods.get_document(&interested));


            let mut method_of_interest:HashSet<String> = HashSet::new();
          
            
            let mut size=object.get_array("interested_methods");
            let mut i_size=0;
            match size{
                Ok(i) =>{
                       i_size=i.len();
                }
                Err(_) => {
                    i_size=0;
                }
            }
            if i_size!=0 {
                for i in 0..i_size {
                    let interested=object.get_array("interested_methods").unwrap()[i].to_string();
                    let item=interested[1..interested.len()-1].to_string();
                        if item!=""{
                            method_of_interest.insert(item);
                        }
                }
            }
            println!("intersting:{:?}",method_of_interest);
            // let func=method_of_interest.get(&0).unwrap();
            // println!("{:?}",methods.get(func));


            let contract_address: ethcontract::H160 = contract_address_string.parse().unwrap();

            let mut read_abi_from: String = String::new();

            match read_abi_result {
                Ok(doc) => {
                    read_abi_from = doc.to_string();
                }
                Err(e) => {
                    println!("ValueNotFound, there is no field of read_abi_from {:?}", e);
                }
            }

            let result: ContractMetaData = ContractMetaData {
                contract_address: contract_address,
                read_abi_from: read_abi_from,
                chain_id: contract_chain_id,
                function_of_interest: function_of_interest,
                contract_name: contract_name,
                contract_description: contract_description,
                contract_slug: contract_slug,
                method_of_interest: method_of_interest,
            };
            println!("The resulting ContractMetadata is {:?}", result);
            Some(result)
        }
        Err(e) => {
            println!("Error in reading contract_meta_data {:?}", e);
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
