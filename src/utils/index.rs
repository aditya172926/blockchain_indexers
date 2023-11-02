use ethcontract::{Http, Instance, H160};
use ethers::abi::Token;
use log::error;
use reqwest::get;
use std::collections::HashMap;
use std::fs::File;
use std::{collections::HashSet, fs};
use web3::Web3;

pub async fn utils_contract_abi(contract_chain_id: &str, contract_address: &str) -> String {
    // println!("The Chain id is {}", contract_chain_id);
    let file: String = fs::read_to_string(r"constants/constants.json")
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
            return "Error in response".to_string();
        }
    }
    // let mut fetched_abi: reqwest::Response = Default::default();
    // return response;
}

pub async fn utils_url_data(param: &str) -> Option<serde_json::Value> {
    let mut query = String::new();
    if param.starts_with("ar://") {
        let arweave_id = &param[5..param.len()];
        query = "https://arweave.net/".to_string() + arweave_id;
    } else if param.starts_with("ipfs://") {
        let ipfs_cid = &param[7..param.len()];
        query = "https://ipfs.io/ipfs/".to_string() + ipfs_cid;
    } else if param.starts_with("https://") {
        query = String::from(param);
    }

    let response = get(query).await;

    match response {
        Ok(object) => {
            let url_data = object
                .text()
                .await
                .expect("Error in parsing nft data to string\n");

            let url_data_content =
                serde_json::from_str(&url_data).expect("error in reading json format");
            return url_data_content;
        }
        Err(error) => {
            error!("Error in fetching nft data from url {:?}\n\n", error);
            return None;
        }
    };
}

pub fn utils_interesting_method(
    method_of_interest: &Vec<std::string::String>,
    method_name: &String,
) -> bool {
    if !method_of_interest.is_empty() {
        return method_of_interest.contains(&method_name.as_str().to_string());
    }
    return true;
}

pub fn utils_contract_instance(
    web3_provider: Web3<Http>,
    contract_abi: web3::ethabi::Contract,
    contract_address: H160,
) -> Instance<Http> {
    Instance::at(web3_provider, contract_abi, contract_address)
}

pub async fn utils_load_fn(handler: String) {
    let path = format!("handlers/{}.rs", handler);
    println!("\n\n\n {}\n\n\n", path);
    let file_path = std::path::Path::new(&path);
    println!("\n\n\n {:?}\n\n\n", file_path);
    let mut file = File::open(path);
    println!("\n\n\n {:?}\n\n\n", file);
}

pub trait TokenToString {
    fn to_string(&self) -> String;
}

impl TokenToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Address(addr) => addr.to_string(),
            Token::FixedBytes(bytes) => format!("{:?}", bytes),
            Token::Bytes(bytes) => format!("{:?}", bytes),
            Token::Int(int) => int.to_string(),
            Token::Uint(uint) => uint.to_string(),
            Token::Bool(boolean) => boolean.to_string(),
            Token::String(string) => string.clone(),
            Token::FixedArray(tokens) => {
                let elements: Vec<String> =
                    tokens.iter().map(|t| ToString::to_string(&t)).collect();
                format!("[{}]", elements.join(", "))
            }
            Token::Array(tokens) => {
                let elements: Vec<String> =
                    tokens.iter().map(|t| TokenToString::to_string(t)).collect();
                format!("[{}]", elements.join(", "))
            }
            Token::Tuple(tokens) => {
                let elements: Vec<String> =
                    tokens.iter().map(|t| ToString::to_string(&t)).collect();
                format!("({})", elements.join(", "))
            }
        }
    }
}
