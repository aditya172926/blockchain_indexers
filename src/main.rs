use ethcontract::prelude::*;
use ethcontract::{
    contract,
    web3::types::{BlockNumber, H160, H256},
    Address
}; 
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use std::fs;
use std::string::String;
use std::{error::Error, str::FromStr};
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;
use futures::join;
use futures::stream::StreamExt;
mod transactions;


// contract!("ens_registry_with_fallback.json");
contract!("abi/abi_1.json");

// type Bytes32 = ArrayVec<u8, 32>

fn main() -> Result<(), Box<dyn Error>> {
    let networkDetails: String = fs::read_to_string(r"config/network.json")?.parse()?;
    let networkDetails = serde_json::from_str::<serde_json::Value>(&networkDetails);
    let networkEndpoint = match networkDetails {
        Ok(object) => object["1"]["networkRpcUrl"].to_string(),
        Err(e) => e.to_string(),
    };
    // println!("{:?}", &networkEndpoint[1..networkEndpoint.len() - 1]);
    
    // println!("{:?}",networkDetails);
    // let networkEndpoint: String = networkDetails
    let networkEndpoint = &networkEndpoint[1..networkEndpoint.len() - 1];

    let contractDetails: String = fs::read_to_string(r"config/global.json")?.parse()?;
    let contractDetails = serde_json::from_str::<serde_json::Value>(&contractDetails);
    let contractChainId;
    let mut contractAddress = "".to_string();
    match contractDetails {
        Ok(object) => {
            contractAddress = object["ens"]["contract_address"].to_string();
            contractChainId = object["ens"]["chainId"].to_string()
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
    contractAddress = contractAddress[1..contractAddress.len() - 1].to_string();
    println!("{:?}", contractAddress);
    let contractAddress =
        Address::from_str(&contractAddress).expect("Failed to convert to address type");

    // temporarily commenting the get_logs function
    // get_logs(&networkEndpoint, contractAddress);

    transactions::get_transaction_data();

    Ok(())
}

//Function to get Logs of events
#[tokio::main]
async fn get_logs(networkEndpoint: &str, contractAddress: Address) -> Result<(), Box<dyn Error>> {
    let transport = Http::new(&networkEndpoint)?;
    let web3 = Web3::new(transport);
    let contract_address: H160 = contractAddress;

    // let _ = fetch_ens_name().await;
    let contract_abi = include_bytes!("../abi/abi_1.json");
    let contract = ENSRegistryWithFallback::at(&web3, contract_address);
    println!("Contract Abi {:?}", contract);
    println!("--------------------------------------------------------");
    println!("1. Contract initialized {:?}", contract);
    // let contract2: Instance = Instance::with_deployment_info(&web3, contract_abi.to_vec(), contract_address, None);
    // Subscribe to all events
    let mut event_streams = contract
        .events()
        .new_owner()
        .stream()
        .boxed();

    loop {
        join! {
            async {
                let log = event_streams.next().await.expect("No events").expect("Error querying event").added();
                let unwraped_log = log.unwrap();
                println!("Received a new event log {:?}", unwraped_log.owner);
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
