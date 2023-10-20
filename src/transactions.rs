use std::collections::HashSet;
use std::str::FromStr;

use ethcontract::Instance;
use ethers::types::H256;
use futures::StreamExt;
use mongodb::bson::Document;
use web3::transports::Http;
use ethers::core::types::Chain;
use ethers::etherscan::Client;
use ethers::prelude::*;

use ethers::{
    prelude::account::Sort,
    providers::Provider
};

use ethers::etherscan::account::TxListParams;

use crate::handlers::ens_ethereum::handler;
use crate::{structs, utils};
use crate::utils::index::utils_interesting_method;
use crate::utils::transactions::utils_transaction_indexed;
use tokio::time::{sleep, Duration};

async fn load_txns(contract_abi:&str,transaction_hash:H256, network_rpc_url:String, methods:Document, method_of_interest:HashSet<String>,contract_slug:String,chain_id:String, contract_address: &str) {
    let mut decoded_txn_data: (
        Vec<structs::index::MethodParam>,  // method params array
        String,                            // method name
        String,                            // method id
        ethers::types::TransactionReceipt, // transaction receipt
    ) = utils::transactions::utils_transaction_data(
        contract_abi,
        transaction_hash,
        &network_rpc_url,
        &methods,
    )
    .await;
  
    if decoded_txn_data.1 != "".to_string()&&utils_interesting_method(&method_of_interest,&decoded_txn_data.1) {
        
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    
        let transaction_indexed = utils_transaction_indexed(&decoded_txn_data,contract_slug,contract_address,chain_id);
        let meta_indexed = handler(&decoded_txn_data.0);   
        // abstractor::create_meta(&contract_slug,transaction_indexed).await;

        // let _ = db::db_transaction_store( 
        //     decoded_txn_data.0, //method_params
        //     decoded_txn_data.1, // function name
        //     decoded_txn_data.2, // function id
        //     decoded_txn_data.3, // transaction receipt
        //     contract_address.clone(),
        //     String::from(&contract_slug),
        //     &chain_id,
        // )
        // .await;
        
        
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }
}


pub async fn get_txns(
    contract_abi: &str,
    contract_instance: &Instance<Http>,
    function_of_interest: String,
    contract_address: String,
    chain_id: String,
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
                println!("//////// TransactionHash /////// \n{:?}",transaction_hash);
                if transaction_hash != prev_txn_hash {
                    load_txns(contract_abi,transaction_hash, network_rpc_url.clone(), methods.clone(), method_of_interest.clone(),contract_slug.clone(),chain_id.clone(),&contract_address);
                    prev_txn_hash = transaction_hash;
                }
                
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

pub async fn get_history(
    contract_address: &str,
    contract_abi: &str,
    start_block: u64,
    end_block: u64,
    chain_id: String,
    contract_slug: String,
    rpc_url: &str,
    api_key: &str,
    methods: Document,
    method_of_interest: HashSet<String>,
    network_rpc_url: String
) -> eyre::Result<()> {
    println!("CHECKING HISTORY...");
    let _provider = Provider::try_from(rpc_url)?;

    //chain was to be generalized *IMPORTANT:CHANGE CHAIN NAME ACCORDING TO CONTRACT*
    println!("The api key is {:?}", api_key);
    let client = Client::builder()
        .with_api_key(api_key)
        .chain(Chain::Polygon)
        .unwrap()
        .build()
        .unwrap();
    println!("for account: {} ", contract_address);
    let paras = TxListParams {
        start_block: start_block,
        end_block: end_block,
        page: 0,
        offset: 0,
        sort: Sort::Asc,
    };
    let mut prev_txn_hash: H256 =
        H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
            .unwrap();
    //Fetching all transactions
    let txns = client
        .get_transactions(&contract_address.parse().unwrap(), Some(paras))
        .await
        .unwrap();
    // println!("{:?}",&txns);

    //Creating loop to iterate over all transactions
    for txn in txns {
        // println!("*************{:?}",txn);
        let from = txn.from;
        let mut s_from: String = String::from("s");
        let mut s_to: String = String::from("s");
        let mut s_contract_used: String = String::from("s");

        //Formatting everything to String
        if let account::GenesisOption::Some(i) = from {
            s_from = i.to_string();
        }
        let to = txn.to;
        if let Some(i) = to {
            s_to = i.to_string();
        }
        // let value = txn.value.to_string();
        // let contract_used = txn.contract_address;

        // let s_contract_used: String = match contract_used {
        //     Some(i) => i.to_string(),
        //     None => String::from("NA"),
        // };
        // let block_number = txn.block_number.to_string();
        // let function_name: String = match txn.function_name {
        //     Some(i) => i.to_string(),
        //     None => String::from("NA"),
        // };
        // let t_hash: String = match txn.hash {
        //     account::GenesisOption::None => String::from("NA"),
        //     account::GenesisOption::Genesis => String::from("0x00"),
        //     account::GenesisOption::Some(i) => i.to_string(),
        // };

        // println!("Sender:{:?},Recipient:{:?}, Value:{:?}, Contract used:{:?}, Block Number:{:?}, Function Used:{}",from,to,value,contract_used,block_number,function_name);


        let txn_hash=txn.hash.value().unwrap().to_owned();
        println!("\n\n\ntrnasaction hash {}\n\n\n", txn_hash);

        if txn_hash != prev_txn_hash {
          load_txns(contract_abi,txn_hash, network_rpc_url.clone(), methods.clone(), method_of_interest.clone(),contract_slug.clone(),chain_id.clone(),&contract_address);
          prev_txn_hash = txn_hash;
        }
    }

    Ok(())
}

