#[allow(unused_imports)]
#[allow(dead_code)]

use std::fmt::Debug;
use std::fs::File;
use std::net::SocketAddr;
use std::error::Error;
use std::fs;
mod getEvents;
use getEvents::getEvents::fetch;

#[allow(unused)]
use ethers::{
    providers::{Provider, Http, Middleware},
    types::{Address, Log, BlockNumber, Filter}, prelude::account::Sort, utils::hex::{self, encode, ToHex},
    abi::Abi
};
use ethcontract::{contract, contract::Event, web3::types::{H160, H256}};
use ethers::core::types::Chain;
use ethers::etherscan::Client;
use ethers::etherscan::account::TxListParams;
use reqwest;
mod nft;
use nft::nft_data::getNftData;
use serde_json::{Value, from_str};



#[tokio::main]
async fn main() -> eyre::Result<()> {


   let contract_address = "0x231d3559aa848Bf10366fB9868590F01d34bF240".to_string();

    let etherscan_api_token="ER9VKT8AXAI2WTPSCRNANN69W67V7PRU59".to_string();


    let api_url=format!("https://api.etherscan.io/api?module=contract&action=getabi&address={}&apikey={}",contract_address,etherscan_api_token);

    let response = reqwest::get(&api_url).await?;

    let mut fetched_abi="na".to_string();


    if response.status().is_success() {
        // Read the response body as a string
        let response_body = response.text().await?;

        // Parse the response body as JSON
        let json: serde_json::Value = serde_json::from_str(&response_body)?;
        
        fetched_abi=json["result"].as_str().unwrap().to_owned();
    } else {
        println!("Request failed with status code: {}", response.status());
    }
    // println!("{:?}",fetched_abi);
    getNftData(contract_address,&fetched_abi).await;    

    
    println!("all okay");
    Ok(())
}


