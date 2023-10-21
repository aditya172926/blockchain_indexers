use ethers::abi::{Abi, Function, Token};
use ethers::types::H160;
use mongodb::bson::document::ValueAccessError;
use mongodb::bson::Document;

use crate::db::{self, index};
use crate::structs::contracts::ContractAbi;
use crate::structs::{contracts::ContractMetaData, networks::NetworkStruct};
use std::collections::HashSet;
use std::fs;
use std::string::String;

pub async fn utils_contract_data(protocol_name: &str) -> (ContractMetaData, ContractAbi) {
    let contract_metadata: ContractMetaData = utils_contract_metadata(protocol_name).await.unwrap();

    println!("The conntract metadata is {:?}", contract_metadata);
    let mut contract_abi_string: String = String::new();
    if contract_metadata.read_abi_from.contains("0x") {
        contract_abi_string = utils_contract_abi(
            &contract_metadata.chain_id,
            &contract_metadata.read_abi_from,
        )
        .await;
    } else {
        contract_abi_string = utils_contract_abi(
            &contract_metadata.chain_id,
            &contract_metadata.contract_address,
        )
        .await;
    }

    let abi_json = serde_json::from_str(&contract_abi_string).unwrap();
    let abi_static: &'static Abi = Box::leak(Box::new(
        serde_json::from_str(&contract_abi_string).expect("Failed to parse abi"),
    ));
    let contract_abi: ContractAbi = ContractAbi {
        string: contract_abi_string,
        raw: abi_json,
        stat: abi_static,
    };
    println!(
        "\n\n\n contract meta data {:?} \n\n contract abi {:?} \n\n\n",
        contract_metadata, contract_abi
    );
    return (contract_metadata, contract_abi);
}

pub async fn utils_contract_metadata(protocol_name: &str) -> Option<ContractMetaData> {
    let contract_result: mongodb::bson::Document = db::index::db_contract_data(protocol_name)
        .await
        .unwrap_or(mongodb::bson::Document::default())
        .clone();
    let contract_meta_data: Result<
        &mongodb::bson::Document,
        mongodb::bson::document::ValueAccessError,
    > = contract_result.get_document("contract");

    let mut methods: &Document = &Document::new();

    let contract_metadata: Option<ContractMetaData> = match contract_meta_data {
        Ok(object) => {
            let mut contract_address_string: String =
                object.get_str("address").unwrap().to_string();
            let contract_address: ethcontract::H160 = contract_address_string.parse().unwrap();

            let mut contract_chain_id: String = object.get_str("chain_id").unwrap().to_string();
            let mut function_of_interest: String = "".to_string();
            let mut contract_name: String = object.get_str("name").unwrap().to_string();
            let mut contract_description: String =
                object.get_str("description").unwrap().to_string();
            let mut contract_slug: String = object.get_str("slug").unwrap().to_string();

            // logic for read_abi_from
            let mut read_abi_result: Result<&str, mongodb::bson::document::ValueAccessError> =
                object.get_str("read_abi_from");

            let read_abi_from: String = match read_abi_result {
                Ok(doc) => doc.to_string(),
                Err(e) => {
                    println!("ValueNotFound, there is no field of read_abi_from {:?}", e);
                    String::new()
                }
            };
            //   wrap();
            // let read_abi_from_h160:H160 = contract_metadata.read_abi_from.parse().unwrap();
            // logic for extracting methods
            methods = match object.get_document("methods") {
                Ok(method_object) => method_object,
                Err(e) => {
                    println!("Value access error in methods {:?}", e);
                    methods
                }
            };

            let mut method_of_interest: HashSet<String> = HashSet::new();

            let mut size: Result<&Vec<mongodb::bson::Bson>, ValueAccessError> =
                object.get_array("interested_methods");
            let mut i_size: usize = 0;
            match size {
                Ok(i) => {
                    i_size = i.len();
                }
                Err(_) => {
                    i_size = 0;
                }
            }
            if i_size != 0 {
                for i in 0..i_size {
                    let interested = object.get_array("interested_methods").unwrap()[i].to_string();
                    let item = interested[1..interested.len() - 1].to_string();
                    if item != "" {
                        method_of_interest.insert(item);
                    }
                }
            }

            // logic to return result
            let result: ContractMetaData = ContractMetaData {
                contract_address: contract_address_string.clone(),
                contract_address_historical: contract_address_string,
                read_abi_from: read_abi_from,
                chain_id: contract_chain_id,
                function_of_interest: function_of_interest,
                contract_name: contract_name,
                contract_description: contract_description,
                contract_slug: contract_slug,
                method_of_interest: method_of_interest,
                methods: methods.clone(),
            };
            Some(result)
        }
        Err(e) => {
            println!("Error in reading contract_meta_data {:?}", e);
            None
        }
    };
    return contract_metadata;
}

pub async fn utils_contract_abi(contract_chain_id: &str, contract_address: &str) -> String {
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
            return String::from("Error ");
        }
    }
}
