use ethcontract::{contract, contract::Event, Address, web3::types::{H160, H256, BlockNumber}};
use futures::StreamExt;
use std::{error::Error, str::FromStr};
use std::string::String;
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;
use sqlx::postgres::{PgPoolOptions,PgRow};
use sqlx::{FromRow,Row};
use std::fs;
use serde::{Deserialize, Serialize};
use ethcontract::prelude::*;



// contract!("ens_registry_with_fallback.json");
contract!("abi/abi_1.json");


 fn main() -> Result<(), Box<dyn Error>> {
    let networkDetails:String = fs::read_to_string(r"/home/nksajwani/metawork-labs/indexer-database/config/network.json")?.parse()?;
    let networkDetails = serde_json::from_str::<serde_json::Value>(&networkDetails);
    let networkEndpoint = match networkDetails {
        Ok(object) => object["1"]["networkRpcUrl"].to_string(),
        Err(e) => e.to_string()
    };
    // println!("{:?}", &networkEndpoint[1..networkEndpoint.len() - 1]);
    
    
    // println!("{:?}",networkDetails);
    // let networkEndpoint: String = networkDetails
    let networkEndpoint = &networkEndpoint[1..networkEndpoint.len() - 1];

    let contractDetails: String = fs::read_to_string(r"/home/nksajwani/metawork-labs/indexer-database/config/global.json")?.parse()?;
    let contractDetails = serde_json::from_str::<serde_json::Value>(&contractDetails);
    let contractChainId;
    let mut contractAddress = "".to_string();
    match contractDetails {
        Ok(object) => {
            contractAddress = object["ens"]["contract_address"].to_string();
            contractChainId = object["ens"]["chainId"].to_string()
        },
        Err(e) => {println!("{:?}", e);}
    };
    contractAddress = contractAddress[1..contractAddress.len() - 1].to_string();
    println!("{:?}", contractAddress);
    let contractAddress = Address::from_str(&contractAddress).expect("Failed to convert to address type");
    get_logs(&networkEndpoint, contractAddress);


     Ok(())
}


//Function to get Logs of events
#[tokio::main]
async fn get_logs(networkEndpoint: &str, contractAddress: Address)-> Result<(), Box<dyn Error>>{
    
    let transport = Http::new(&networkEndpoint)?;
    let web3 = Web3::new(transport);
    let contract_address: H160 = contractAddress;

    // let _ = fetch_ens_name().await;

    let contract = ENSRegistryWithFallback::at(&web3, contract_address);
    //Subscribe to all events
    let event_stream = contract.all_events().from_block(BlockNumber::from(17547614)).stream();
    
    let mut event_stream = Box::pin(event_stream);

    loop {
        match event_stream.next().await {
            Some(Ok(log)) => {

                // Handle the event
                println!("Received event: {:?}", log.data);
                // println!("{:?}", &log.added().unwrap());
                let to_address=log.meta.as_ref().unwrap().address.to_string();
                // let to_address=log.meta.as_ref().unwrap().address.to_string();
                let block_no:i64=log.meta.as_ref().unwrap().block_number.try_into().unwrap();
                let txn_hash:String=log.meta.as_ref().unwrap().transaction_hash.to_string();
                println!("TO Address: {:?}", &to_address);
                println!("Block Number: {:?}", &block_no);
                println!("Transaction Hash: {}", txn_hash);
                // add_to_db(to_address,block_no,txn_hash).await?;
                // println!("Received event: {:?}", log);
                
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
            }
            None => {
                println!("Stream ended, reconnecting...");
                sleep(Duration::from_secs(2)).await;
                
                event_stream = Box::pin(contract.all_events().from_block(BlockNumber::from(17547614)).stream());
            }
        }
    }
    Ok(())

}



#[derive(Debug,FromRow)]
pub struct MyEvent{
    id:i64,
    // from:String,
    to:String,              //EOA or Contract address
    block_number:i64,
}


async fn add_to_db(to_address:String,block_no:i64,txn_hash:String) -> Result<(),sqlx::Error>{

//Create instance
    println!("Inside add to db");
    let pool=PgPoolOptions::new().max_connections(5).connect("postgres://postgres:1994@localhost/test").await?;
    println!("{:?}", pool);
//Create table
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS event (
        id bigserial,
        "to" text,
        block_number int,
        Txn_Hash text 
        );"#,
        )
        .execute(&pool)
        .await?;

//insert new event


    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO event ( "to", block_number, txn_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#
    )
    .bind(to_address)
    .bind(block_no)
    .bind(txn_hash)
    .fetch_one(&pool)
    .await?;

//Fetch data
    
    // let select_query = sqlx::query_as::<_, MyEvent>(
    //     r#"
    //     SELECT id, "to", block_number
    //     FROM event
    //     "#
    // );

	// let events: Vec<MyEvent> = select_query.fetch_all(&pool).await?;
	// println!("\n=== select events with query.map...: \n{:?}", events);

    Ok(())
}

