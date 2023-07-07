use ethcontract::contract;
use ethcontract::contract::Instance;
use ethcontract::prelude::*;
use ethers::{
    abi::Abi,
    contract::Contract,
    types::{Address, H160, U256},
};

use ethers::providers::Provider;
use futures::join;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::fs;
use std::string::String;
use std::sync::Arc;
use std::{error::Error, str::FromStr};
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;
mod utils;
mod transactions;

// contract!("ens_registry_with_fallback.json");
contract!("abi/abi_1.json");

// type Bytes32 = ArrayVec<u8, 32>
fn main() -> Result<(), Box<dyn Error>> {
    let network_endpoint: String = utils::get_network_rpc("1");
    let contract_address: String = utils::get_contract_metadata(&"uniswap".to_string());
    let fetched_abi: String = initialize_node(&network_endpoint, &contract_address);
    let _ = get_logs(network_endpoint, &contract_address, fetched_abi);
    // transactions::get_transaction_data(&fetched_abi, "post".to_string(), "0x6b69174c0969eda83feb75734fee22722b518aba79be76aaa839ae58fd44d58b".to_string());
    Ok(())
}

#[tokio::main]
async fn initialize_node(network_endpoint: &str, contract_address:&str) -> String {
    let transport: Http = Http::new(&network_endpoint).expect("Error");
    let web3: Web3<Http> = Web3::new(transport);

    let response: Result<reqwest::Response, reqwest::Error> = utils::fetch_contract_abi("mainnet".to_string(), &contract_address).await;
    // let contract_abi: Result<String, reqwest::Error>;
    let mut fetched_abi: String = String::new();

    match response{
        Ok(object) => {
            if object.status().is_success() {
                // Read the response body as a string
                let response_body: String = object.text().await.expect("Error");
                // Parse the response body as JSON
                let json: serde_json::Value = serde_json::from_str(&response_body).expect("error");
                fetched_abi = json["result"].as_str().unwrap().to_owned();
            } else {
                println!("Request failed with status code: {}", object.status());
            }
            return fetched_abi;
        }
        Err(e) => {
            println!("Error in fetching contract abi {:?}", e);
            return "Error".to_string();
        }
    }
}

#[tokio::main]
async fn get_logs(network_endpoint: String, contract_address: &str, fetched_abi:String) -> Result<(), Box<dyn Error>> {
    let transport: Http = Http::new(&network_endpoint)?;
    let web3: Web3<Http> = Web3::new(transport);
    let abi = serde_json::from_str(&fetched_abi).unwrap();
    println!("The contract abi unwrapped is {:?}", abi);
    // ----------------------------------------------------------------------

    let address: ethcontract::H160 = contract_address.parse()?;
    let contract_instance = Instance::at(web3, abi, address);
    println!("Contract address {:?}", address);
    println!("--------------------------------------------------------");
    println!("1. Contract initialized");

    // Subscribe to all events
    let mut event_streams = contract_instance
        .all_events()
        .from_block(BlockNumber::from(17630615))
        .stream()
        .boxed();

    println!("waiting for events.......");
    loop {
        join! {
            async {
                let log = event_streams.next().await.expect("No events").expect("Error querying event").added();
                let unwraped_log = log.unwrap();
                println!("Received a new event log {:?}", unwraped_log);


                let to=&unwraped_log.topics[2];
                println!("Recipient Address:{:?}",to);

                let amount=ethers::types::U256::from_big_endian(&unwraped_log.data[..32]);
                println!("Amount:{:?}",amount);

            },
        };
    }

    Ok(())
}

// #[derive(Debug, FromRow)]
// pub struct MyEvent {
//     id: i64,
//     // from:String,
//     to: String, //EOA or Contract address
//     block_number: i64,
// }

// // async fn add_to_db(to_address:String,block_no:i64,txn_hash:String) -> Result<(),sqlx::Error>{

// // //Create instance
// //     println!("Inside add to db");
// //     let pool=PgPoolOptions::new().max_connections(5).connect("postgres://postgres:1994@localhost/test").await?;
// //     println!("{:?}", pool);
// // //Create table
// //         sqlx::query(
// //             r#"
// //         CREATE TABLE IF NOT EXISTS event (
// //         id bigserial,
// //         "to" text,
// //         block_number int,
// //         Txn_Hash text
// //         );"#,
// //         )
// //         .execute(&pool)
// //         .await?;

// // //insert new event

// //     let row: (i64,) = sqlx::query_as(
// //         r#"
// //         INSERT INTO event ( "to", block_number, txn_hash)
// //         VALUES ($1, $2, $3)
// //         RETURNING id
// //         "#
// //     )
// //     .bind(to_address)
// //     .bind(block_no)
// //     .bind(txn_hash)
// //     .fetch_one(&pool)
// //     .await?;

// // //Fetch data

// //     // let select_query = sqlx::query_as::<_, MyEvent>(
// //     //     r#"
// //     //     SELECT id, "to", block_number
// //     //     FROM event
// //     //     "#
// //     // );

// // 	// let events: Vec<MyEvent> = select_query.fetch_all(&pool).await?;
// // 	// println!("\n=== select events with query.map...: \n{:?}", events);

// //     Ok(())
// // }
