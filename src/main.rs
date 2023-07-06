use ethcontract::prelude::*;
use ethcontract::{
    contract,
}; 
use ethers::{
    contract::Contract,
    types::{Address, U256,H160},
    abi::Abi
};

use ethers::providers::Provider;
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
    println!("--------------------------------------------------------------");
    get_logs(&networkEndpoint, contractAddress);

    // transactions::get_transaction_data();

    Ok(())
}

//Function to get Logs of events
#[tokio::main]
async fn get_logs(networkEndpoint: &str, contractAddress: Address) -> Result<(), Box<dyn Error>> {
    // let transport = Http::new(&networkEndpoint)?;
    // let web3 = Web3::new(transport);
    // let contract_address: H160 = contractAddress;
    let rpc_url = "https://lingering-delicate-choice.discover.quiknode.pro/68f9e3726efe97ee2b6a7c8417f6f5d12ab713c6/";
    let provider = Provider::try_from(rpc_url)?;


    let etherscan_api_token="ER9VKT8AXAI2WTPSCRNANN69W67V7PRU59".to_string();
    let contract_address = "0x231d3559aa848Bf10366fB9868590F01d34bF240".to_string();


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

    let abi: Abi = serde_json::from_str(&fetched_abi).unwrap();
    let address:H160=contract_address.parse()?;


    let contract_instance=Contract::new(address,abi,Arc::new(provider.clone()));
    println!("{:?}",contract_instance);



    // let _ = fetch_ens_name().await;
    // let contract_abi = include_bytes!("../abi/abi_1.json");
    // let contract = ENSRegistryWithFallback::at(&web3, contract_address);

    println!("Contract Abi {:?}", address);
    println!("--------------------------------------------------------");
    println!("1. Contract initialized {:?}", contract_instance);
    // let contract2: Instance = Instance::with_deployment_info(&web3, contract_abi.to_vec(), contract_address, None);
    // Subscribe to all events
    // let mut event_streams = contract_instance
    //     .events()
    //     .new_owner()
    //     .stream()
    //     .boxed();

//     let logs = contract_instance
//     .event()
//     .from_block(0u64)
//     .query()
//     .await?;

// println!("{:?}", logs);

    // loop {
    //     join! {
    //         async {
    //             let log = event_streams.next().await.expect("No events").expect("Error querying event").added();
    //             let unwraped_log = log.unwrap();
    //             println!("Received a new event log {:?}", unwraped_log.owner);
    //         },
    //     };
    // }

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
