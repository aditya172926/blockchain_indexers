

mod ens;            //Custom module to use ens program
use ens::ens::fetch_ens_name;
use ethcontract::{contract, contract::Event, web3::types::{H160, H256, BlockNumber}};
use futures::StreamExt;
use std::error::Error;
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;
use sqlx::postgres::{PgPoolOptions,PgRow};
use sqlx::{FromRow,Row};



// contract!("ens_registry_with_fallback.json");
contract!("ENSRegistryWithFallback.json");


 fn main() -> Result<(), Box<dyn Error>> {
    

     get_logs();

    // Configure your Ethereum network provider and contract
    // let transport = Http::new("https://goerli.infura.io/v3/c108a133505241de9e2c48894d23e483")?;
    // let web3 = Web3::new(transport);
    // let contract_address: H160 = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e".parse()?;
    // let contract = ENSRegistryWithFallback::at(&web3, contract_address);
    // // Subscribe to all events
    // let event_stream = contract.all_events().from_block(BlockNumber::from(9103406)).stream();
    // let mut event_stream = Box::pin(event_stream);
    // loop {
    //     match event_stream.next().await {
    //         Some(Ok(log)) => {
    //             // Handle the event
    //             println!("Received event: {:?}", log);
    //         }
    //         Some(Err(e)) => {
    //             eprintln!("Error: {}", e);
    //         }
    //         None => {
    //             println!("Stream ended, reconnecting...");
    //             sleep(Duration::from_secs(2)).await;   
    //             event_stream = Box::pin(contract.all_events().from_block(BlockNumber::from(9103406)).stream());
    //         }
    //     }
    // }
     

     Ok(())
}


//Function to get Logs of events
#[tokio::main]
async fn get_logs()-> Result<(), Box<dyn Error>>{

    let transport = Http::new("https://mainnet.infura.io/v3/c108a133505241de9e2c48894d23e483")?;
    let web3 = Web3::new(transport);
    let contract_address: H160 = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e".parse()?;

    // let _ = fetch_ens_name().await;

    let contract = ENSRegistryWithFallback::at(&web3, contract_address);
    // Subscribe to all events
    let event_stream = contract.all_events().from_block(BlockNumber::from(17547614)).stream();
    let mut event_stream = Box::pin(event_stream);

    loop {
        match event_stream.next().await {
            Some(Ok(log)) => {

                // Handle the event
                // println!("Received event: {:?}", log.data);
                // println!("{:?}", &log.added().unwrap());
                let to_address=log.meta.as_ref().unwrap().address.to_string();
                let block_no:i64=log.meta.as_ref().unwrap().block_number.try_into().unwrap();
                let txn_hash:String=log.meta.as_ref().unwrap().transaction_hash.to_string();
                println!("TO Address: {:?}", &to_address);
                println!("Block Number: {:?}", &block_no);
                println!("Transaction Hash: {:?}", &log.meta.as_ref().unwrap().transaction_hash);
                add_to_db(to_address,block_no,txn_hash).await?;
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
    let pool=PgPoolOptions::new().max_connections(5).connect("postgres://postgres:g[@password]@localhost/test").await?;

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

