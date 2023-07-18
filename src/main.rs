use ethcontract::contract::Instance;
use ethcontract::prelude::*;
use ethers::types::H256;
use futures::join;
use futures::stream::StreamExt;
use std::string::String;
use std::collections::HashSet;
use std::{error::Error, str::FromStr};
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;

// modules
mod db;
mod middleware; 
mod structs;
mod transactions;
mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let contract_result: (structs::ContractMetaData, String, web3::ethabi::Contract) =
        utils::get_contract_data("ens_ethereum").await;

    let contract_metadata = contract_result.0;
    let contract_fetched_abi = contract_result.1;
    let contract_abi = contract_result.2;


    let network_metadata:structs::NetworkMetaData = utils::get_network_data(&contract_metadata.chain_id).unwrap();

    
    println!("The contract ABI is {:?}", contract_abi);
    let transport: Http = Http::new(&network_metadata.network_rpc_url)?;
    let web3: Web3<Http> = Web3::new(transport);
    let contract_instance: Instance<Http> = Instance::at(web3, contract_abi, contract_metadata.contract_address);

    get_txns(
        &contract_fetched_abi,
        &contract_instance,
        contract_metadata.function_of_interest,
        contract_metadata.contract_address.to_string(),
        contract_metadata.chain_id,
        contract_metadata.contract_name,
        contract_metadata.contract_description,
        contract_metadata.contract_slug,
        network_metadata.network_rpc_url,
        network_metadata.start_block_number,
        contract_metadata.method_of_interest
    )
    .await;

    // let _ = get_logs(contract_instance, 17630615).await;

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
    method_of_interest:HashSet<String>
) {

    println!("The RPC is {}", network_rpc_url);

    let contract_data: structs::ContractData = structs::ContractData {
        address: String::from(&contract_address),
        chain_id: chain_id,
        name: String::from(&contract_name),
        description: String::from(contract_description),
        slug: String::from(&contract_slug),
        image: String::from(""),
        interested_methods: vec![function_of_interest],
        interested_events: vec!["".to_string()],
    };

    // let _ = db::save_contract_to_db(contract_data).await;


// eth block number:17691422
//polygon block number:45033964
    let event_stream = contract_instance
        .all_events()
        .from_block(BlockNumber::from(network_block_number))
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

                let decoded_txn_data: (
                    Vec<structs::MethodParam>,
                    String,
                    String,
                    ethers::types::TransactionReceipt,
                ) = transactions::get_transaction_data(contract_abi, transaction_hash, &network_rpc_url).await;

                let current_txn_hash: H256 = decoded_txn_data.3.transaction_hash;

                if current_txn_hash != prev_txn_hash && decoded_txn_data.1 != "".to_string() {

                    if is_interesting_method(&method_of_interest,&decoded_txn_data.1) {

                        
                        // let _ = db::save_txn_to_db(
                        //     decoded_txn_data.0,
                        //     decoded_txn_data.1,
                        //     decoded_txn_data.2,
                        //     decoded_txn_data.3,
                        //     String::from(&contract_address),
                        //     String::from(&contract_slug),
                        // )
                        // .await;
                    println!("Added txn:{:?}", current_txn_hash);
                    }
                    prev_txn_hash = current_txn_hash;
                }
                println!(
                    "============================================================================="
                );
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
                        .from_block(BlockNumber::from(network_block_number))
                        .stream(),
                );
            }
        }
    }
}




fn is_interesting_method(method_of_interest:&HashSet<String>,method_name:&String)-> bool{
    println!("{:?}",method_of_interest);
    if !method_of_interest.is_empty(){
        return method_of_interest.contains(method_name.as_str());
    }
    return true;
}



async fn get_logs(
    contract_instance: Instance<Http>,
    block_number: i64,
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
                let unwrapped_log = log.unwrap();
                // let _ = db::save_to_db(unwrapped_log).await;
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
