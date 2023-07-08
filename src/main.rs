use ethcontract::contract;
use ethcontract::contract::Instance;
use ethcontract::prelude::*;
use ethers::{
    abi::Abi,
    contract::Contract,
    types::{Address, H160, U256, H256},
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

// modules
mod transactions;
mod utils;
mod middleware;

// contract!("ens_registry_with_fallback.json");


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let contract_metadata: (String, String, String) = utils::get_contract_metadata(&"opensea_ethereum".to_string());
    let contract_address: String = contract_metadata.0;
    let mut contract_chain_id: String = contract_metadata.1;
    contract_chain_id = contract_chain_id[1..contract_chain_id.len()-1].to_string();
    let function_of_interest: String = contract_metadata.2;
    // println!("contract_chain_id in main is {}", &contract_chain_id);
    let network_rpc: String = utils::get_network_rpc(&contract_chain_id);

    let contract_fetched_abi: String = utils::format_contract_abi(&contract_chain_id, &contract_address).await;
    let contract_address_h160: ethcontract::H160 = contract_address.parse()?;

    let contract_abi: web3::ethabi::Contract = serde_json::from_str(&contract_fetched_abi).unwrap(); 
    // println!("The abi functions are {:?}", abi.functions);
    // for function in &abi.functions {
    //     println!("The contract function is {:?}", function); // prints the list of functions
    // }
    let transport: Http = Http::new(&network_rpc)?;
    let web3: Web3<Http> = Web3::new(transport); 
    let contract_instance: Instance<Http> = Instance::at(web3, contract_abi, contract_address_h160);

    get_txns(&contract_fetched_abi, &contract_instance).await;

    let _ = get_logs(contract_instance, 17630615);

    Ok(())
}


async fn get_txns(contract_abi: &str, contract_instance: &Instance<Http>){
    let event_stream = contract_instance.all_events().from_block(BlockNumber::from(17547614)).stream();
    println!("fetching...");
    let mut event_stream = Box::pin(event_stream);
   
    loop {
        match event_stream.next().await {
            Some(Ok(log)) => {


                // Handle the event
                println!("Received event: {:?}", log);
                // println!("{:?}", &log.added().unwrap());
                let to_address=log.meta.as_ref().unwrap().address.to_string();
                // let to_address=log.meta.as_ref().unwrap().address.to_string();
                let block_no:i64=log.meta.as_ref().unwrap().block_number.try_into().unwrap();
                let txn_hash=log.meta.as_ref().unwrap().transaction_hash.to_fixed_bytes();
                let txnr: H256=ethers::core::types::TxHash::from(txn_hash);
                
                println!("TO Address: {:?}", &to_address);
                println!("Block Number: {:?}", &block_no);
                println!("Transaction Hash: {:?}", txnr);
                let decoded_txn_data: (Vec<ethers::abi::Token>, String) = transactions::get_transaction_data(contract_abi, txnr).await;
                middleware::check_transaction_data(decoded_txn_data);
                // add_to_db(to_address,block_no,txn_hash).await?;
                // println!("Received event: {:?}", log);
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
            }
            None => {
                println!("Stream ended, reconnecting...");
                sleep(Duration::from_secs(2)).await;

                event_stream = Box::pin(
                    contract_instance
                        .all_events()
                        .from_block(BlockNumber::from(17547614))
                        .stream(),
                );
            }
        }
    }
}

#[tokio::main]
async fn get_logs(
    contract_instance: Instance<Http>,
    block_number: i64
) -> Result<(), Box<dyn Error>> {
    // Subscribe to all events
    let mut event_streams = contract_instance
        .all_events()
        .from_block(BlockNumber::from(block_number))
        .stream()
        .boxed();

    println!("waiting for events.......");
    loop {
        join! {
            async {
                let log = event_streams.next().await.expect("No events").expect("Error querying event").added();
                let unwraped_log = log.unwrap();
                println!("Received a new event log {:?}", unwraped_log);
                for topic in &unwraped_log.topics {
                    println!("Logging topic {:?}", topic);
                }
            },
        };
    }
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
