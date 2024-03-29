use crate::structs::{
    contracts::{ContractAbi, ContractEvent, ContractEventMap, ContractIndexed, ContractMetaData},
    extract::{Config, Schema},
};
use ethers::abi::{Abi, Function, Token};
use ethers::contract::{Contract, ContractInstance};
use ethers::providers::{Http, Provider};
use ethers::types::{H160, H256};
use log::{debug, error, info, warn};
use std::{collections::HashMap, fs};
use std::{string::String, sync::Arc};

pub async fn utils_contract_list(
    client: &Arc<Provider<Http>>,
    schema: &Schema,
) -> Vec<ContractIndexed> {
    let mut contracts: Vec<ContractIndexed> = vec![];
    let length: usize = schema.source.len();

    for i in 0..length {
        let contract: ContractIndexed = utils_contract_data(&client, i, &schema).await;
        contracts.push(contract);
    }
    return contracts;
}

pub async fn utils_contract_data(
    client: &Arc<Provider<Http>>,
    sourceIndex: usize,
    schema: &Schema,
) -> ContractIndexed {
    let mut interested_events: Vec<ContractEvent> = vec![];
    let mut interested_events_map: HashMap<H256, String> = HashMap::new();
    let mut interested_event_topics: Vec<H256> = vec![];

    for event in schema.source[sourceIndex].interestedEvents.iter() {
        let topic: H256 = event.topic0.parse().unwrap();
        let e: ContractEvent = ContractEvent {
            topic0: topic,
            name: event.name.clone(),
        };
        interested_event_topics.push(topic);
        interested_events_map.insert(topic, event.name.clone());
        interested_events.push(e);
    }

    let contract_events: ContractEventMap = ContractEventMap {
        topics: interested_event_topics,
        map: interested_events_map,
        events: interested_events,
    };

    let contract_metadata: ContractMetaData = ContractMetaData {
        contract_address: schema.source[sourceIndex].from.to_owned(),
        contract_address_H160: schema.source[sourceIndex].from.to_owned().parse().unwrap(),
        contract_address_historical: schema.source[sourceIndex].fromHistorical.to_owned(),
        contract_address_historical_H160: schema.source[sourceIndex]
            .fromHistorical
            .to_owned()
            .parse()
            .unwrap(),
        read_abi_from: schema.source[sourceIndex].readAbiFrom.to_owned(),
        read_abi_from_H160: schema.source[sourceIndex]
            .readAbiFrom
            .to_owned()
            .parse()
            .unwrap(),
        chain_id: schema.source[sourceIndex].networkId.to_owned(),
        method_of_interest: schema.source[sourceIndex].interestedMethods.to_owned(),
        events_of_interest: contract_events,
    };

    let contract_abi_string: String = utils_contract_abi(&contract_metadata).await;
    let abi_json: web3::ethabi::Contract = serde_json::from_str(&contract_abi_string).unwrap();
    let abi_static: ethers::core::abi::Abi = serde_json::from_str(&contract_abi_string).unwrap();
    let contract_abi: ContractAbi = ContractAbi {
        string: contract_abi_string,
        raw: abi_json,
        stat: abi_static,
    };

    let contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>> = Contract::new(
        contract_metadata.contract_address_H160,
        contract_abi.stat.clone(),
        client.clone(),
    );

    let contract_indexed: ContractIndexed = ContractIndexed {
        data: contract_metadata,
        abi: contract_abi,
        instance: contract_instance,
    };

    return contract_indexed;
}

pub async fn utils_contract_abi(contract_metadata: &ContractMetaData) -> String {
    let file: String = fs::read_to_string(r"constants/constants.json")
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
