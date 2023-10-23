use ethers::abi::{Abi, Function, Token};
use ethers::types::H160;
use mongodb::bson::document::ValueAccessError;
use mongodb::bson::Document;

use crate::db::{self, index};
use crate::structs::contracts::ContractAbi;
use crate::structs::extract::Config;
use crate::structs::{contracts::ContractMetaData, networks::NetworkStruct};
use log::{debug, error, info, warn};
use std::collections::HashSet;
use std::fs;
use std::string::String;

pub async fn utils_contract_data(config: &Config) -> (ContractMetaData, ContractAbi) {
    let contract_metadata: ContractMetaData = ContractMetaData {
        contract_address: config.source[0].from.to_owned(),
        contract_address_historical: config.source[0].fromHistorical.to_owned(),
        read_abi_from: config.source[0].readAbiFrom.to_owned(),
        chain_id: config.source[0].networkId.to_owned(),
        start_block: config.source[0].startBlock.to_owned(),
        end_block: config.source[0].endBlock.to_owned(),
        method_of_interest: config.source[0].interestedMethods.to_owned(),
    };

    let mut contract_abi_string: String = utils_contract_abi(&contract_metadata).await;

    let abi_json = serde_json::from_str(&contract_abi_string).unwrap();
    let abi_static: &'static Abi = Box::leak(Box::new(
        serde_json::from_str(&contract_abi_string).expect("Failed to parse abi"),
    ));
    let contract_abi: ContractAbi = ContractAbi {
        string: contract_abi_string,
        raw: abi_json,
        stat: abi_static,
    };
    return (contract_metadata, contract_abi);
}

pub async fn utils_contract_abi(contract_metadata: &ContractMetaData) -> String {
    let file: String = fs::read_to_string(r"config/constants.json")
        .expect("Error in reading the constants.json file");
    let file_data = serde_json::from_str::<serde_json::Value>(&file);

    let mut api: String = String::new();
    match file_data {
        Ok(object) => {
            api = object[contract_metadata.chain_id.to_string()]["_api"].to_string();
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }

    let mut api_url = str::replace(&api, "{}", &contract_metadata.read_abi_from);
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
                error!("Request failed with status code: {}", object.status());
            }

            return fetched_abi;
        }
        Err(e) => {
            println!("Error in fetching contract abi {:?}", e);
            return String::from("Error ");
        }
    }
}
