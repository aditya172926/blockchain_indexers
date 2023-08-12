use ethcontract::contract::Instance;
use ethcontract::prelude::*;
use ethers::providers::Provider;
use ethers::types::{H256, Filter};
use futures::join;
use futures::stream::StreamExt;
use hex::ToHex;
use mongodb::bson::document::ValueAccessError;
use mongodb::bson::Document;
use std::collections::HashSet;
use std::string::String;
use std::{error::Error, str::FromStr};
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;

// modules
mod db;
mod history;
mod middleware;
mod structs;
mod transactions;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let contract_result: (structs::ContractMetaData, String, web3::ethabi::Contract) =
        utils::get_contract_data("omnia_optimism").await;

    let contract_metadata: structs::ContractMetaData = contract_result.0;
    let contract_fetched_abi: String = contract_result.1;
    let contract_abi: web3::ethabi::Contract = contract_result.2;
    let network_metadata: structs::NetworkMetaData =
        utils::get_network_data(&contract_metadata.chain_id).unwrap();
        println!("{}",network_metadata.network_api_key);
    let transport: Http = Http::new(&network_metadata.network_rpc_url)?;
    let web3: Web3<Http> = Web3::new(transport);

    // println!("The contract ABI is {:?}", contract_abi);
    let contract_address_h160 = contract_metadata.contract_address.parse().unwrap();
    let contract_instance: Instance<Http> =
        Instance::at(web3, contract_abi, contract_address_h160);
        
    let contract_address_string = format!("{:020x}", contract_address_h160);
    let initial = String::from("0x");
    let s_contract_address = format!("{}{}", initial, contract_address_string);

    //for eth:
    //start: 17394000
    //end: 17394604

    //for polygon:
    //start: 45608700
    //end: 45608720

    // let start_block: u64 = 	17086038;
    // let end_block: u64 = 17090591;
    // let _ = history::get_history(
    //     &s_contract_address,
    //     &contract_fetched_abi,
    //     start_block,
    //     end_block,
    //     contract_metadata.chain_id,
    //     contract_metadata.contract_slug,
    //     &network_metadata.network_rpc_url,
    //     &network_metadata.network_api_key,
    //     contract_metadata.methods,
    //     contract_metadata.method_of_interest,
    // )
    // .await;

    get_txns(
        &contract_fetched_abi,
        &contract_instance,
        contract_metadata.function_of_interest,
        s_contract_address,
        contract_metadata.chain_id,
        contract_metadata.contract_name,
        contract_metadata.contract_description,
        contract_metadata.contract_slug,
        network_metadata.network_rpc_url,
        network_metadata.start_block_number,
        contract_metadata.method_of_interest,
        contract_metadata.methods,
    )
    .await;

    // let _ = get_events(contract_instance, 17630615).await;

    Ok(())
}

async fn get_txns(
    contract_abi: &str,
    contract_instance: &Instance<Http>,
    function_of_interest: String,
    contract_address: String,
    chain_id: String,
    contract_name: String,
    contract_description: String,
    contract_slug: String,
    network_rpc_url: String,
    network_block_number: i64,
    method_of_interest: HashSet<String>,
    methods: Document,
) {
    println!("The RPC is {}", network_rpc_url);
    println!("The block number is {:?}", ethcontract::BlockNumber::from(network_block_number));

    // eth block number:17691422
    //polygon block number:45033964
    let event_stream = contract_instance
        .all_events()
        .from_block(ethcontract::BlockNumber::from(network_block_number))
        .stream();
    println!("fetching...");
    let mut event_stream = Box::pin(event_stream);
    let mut prev_txn_hash: H256 =
        H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
            .unwrap();

    println!("Trying...");
    loop {

        match event_stream.next().await {
            Some(Ok(log)) => {

                let txn_hash = log.meta.as_ref().unwrap().transaction_hash.to_fixed_bytes();
                let transaction_hash: H256 = ethers::core::types::TxHash::from(txn_hash);
                println!("{:?}",transaction_hash);

                if transaction_hash != prev_txn_hash {
                    let mut decoded_txn_data: (
                        Vec<structs::MethodParam>,         // method params array
                        String,                            // function name
                        String,                            // transaction hash
                        ethers::types::TransactionReceipt, // transaction receipt
                    ) = transactions::get_transaction_data(
                        contract_abi,
                        transaction_hash,
                        &network_rpc_url,
                        &methods,
                    )
                    .await;

                    if decoded_txn_data.1 != "".to_string() {
                        if decoded_txn_data.0.len() > 1
                            && decoded_txn_data.0[1].name == "owner"
                            && &decoded_txn_data.0[1].kind == "address"
                        {
                            let onwer_value = &decoded_txn_data.0[1].value;
                            let initial = String::from("0x");
                            decoded_txn_data.0[1].value = format!("{:?}{:?}", initial, onwer_value);

                            println!(
                                "AFTER====================name:{:?},kind:{:?},value:{:?}=================",
                                decoded_txn_data.0[1].name,
                                decoded_txn_data.0[1].kind,
                                decoded_txn_data.0[1].value
                            );
                        }

                        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                        println!("{:?}", decoded_txn_data);
                        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
                        if is_interesting_method(&method_of_interest,&decoded_txn_data.1) {
                        let _ = db::save_txn_to_db(
                            decoded_txn_data.0, //method_params
                            decoded_txn_data.1, // function name
                            decoded_txn_data.2, // function id
                            decoded_txn_data.3, // transaction receipt
                            contract_address.clone(),
                            String::from(&contract_slug),
                            &chain_id,
                        )
                        .await;
                        println!("Added txn:{:?}", transaction_hash);
                    }
                        // println!("{:?}",decoded_txn_data);
                        // println!("{:?}",decoded_txn_data);
                        println!("cont_add txn:{:?}", contract_address.clone());
                        prev_txn_hash = transaction_hash;
                    }
                }
                println!(
                    "============================================================================="
                );
            }
            Some(Err(e)) => {
                println!("Error: {}", e);
            }
            None => {
                println!("Stream ended, reconnecting...");
                sleep(Duration::from_secs(2)).await;

                event_stream = Box::pin(
                    contract_instance
                        .all_events()
                        .from_block(ethcontract::BlockNumber::from(network_block_number))
                        .stream(),
                );
            }
        }
    }
}

async fn get_events(
    contract_instance: Instance<Http>,
    block_number: i64,
) -> Result<(), Box<dyn Error>> {
    // Subscribe to all events
    let mut event_streams = contract_instance
        .all_events()
        .from_block(ethcontract::BlockNumber::from(block_number))
        .stream()
        .boxed();

    println!("waiting for events.......");
    loop {
        join! {
            async {
                let log = event_streams.next().await.expect("No events").expect("Error querying event").added();
                let unwrapped_log = log.unwrap();
                // let _ = db::save_to_db(unwrapped_log).await;
            },
        };
    }
}


fn is_interesting_method(method_of_interest:&HashSet<String>,method_name:&String)-> bool{
    if !method_of_interest.is_empty(){
        return method_of_interest.contains(method_name.as_str());
    }
    return true;
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
