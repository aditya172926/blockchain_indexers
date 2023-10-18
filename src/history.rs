use std::collections::HashSet;
use std::str::FromStr;

use chrono::Utc;
use ethers::core::types::Chain;
use ethers::etherscan::account::TxListParams;
use ethers::etherscan::Client;
use ethers::prelude::*;
use ethers::{
    prelude::account::Sort,
    providers::Provider
};
use mongodb::bson::{Document, to_bson};

use crate::structs::{TransactionIndexed, Transaction};
use crate::{structs, transactions,db, abstractor};

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

        if txn_hash != prev_txn_hash {

            let mut decoded_txn_data: (
                Vec<structs::MethodParam>,         // method params array
                String,                            // function name
                String,                            // transaction hash
                ethers::types::TransactionReceipt, // transaction receipt
            ) = transactions::utils_transaction_data(
            contract_abi,
            txn_hash,
            &rpc_url,
            &methods,
        )
        .await;
        println!("{:?}",decoded_txn_data);

    if decoded_txn_data.1 != "".to_string() {
        if is_interesting_method(&method_of_interest,&decoded_txn_data.1) {

            let block_number_option=decoded_txn_data.3.block_number;
            let block_number = match block_number_option {
                Some (object) => object.as_u64(),
                None => 0
            };
            // let block_number=transaction_receipt.block_number.unwrap().to_string();

            let transaction_struct: Transaction = Transaction {
                block_hash: decoded_txn_data.3.block_hash,
                block_number:block_number,
                contract_slug:contract_slug.clone(),
                contract_address: contract_address.clone().to_owned(),
                chain_id: chain_id.to_string(),
                gas_used: decoded_txn_data.3.gas_used,
                gas_price: decoded_txn_data.3.effective_gas_price,
                from: decoded_txn_data.3.from,
                to: decoded_txn_data.3.to,
                txn_hash: decoded_txn_data.3.transaction_hash,
                method_name: decoded_txn_data.1,
                method_id: decoded_txn_data.2,
                method_params: decoded_txn_data.0,
            };    
          

            let now = Utc::now();
            let ts: String = now.timestamp().to_string();
            println!("Current timestamp is: {}", ts);


            // let event_bson: mongodb::bson::Bson = to_bson(&txn).unwrap();
            let transaction_bson_receipt: mongodb::bson::Bson = to_bson(&transaction_struct).unwrap();
            let event_document: TransactionIndexed = TransactionIndexed {
                timestamp: ts,
                transaction: transaction_struct,
            };
            println!("\n\nThe event document is {:?}\n\n", event_document);
            println!("Creating meta now");

            abstractor::create_meta(&contract_slug,event_document).await;





    // let _ = db::db_transaction_store(
    //         decoded_txn_data.0, //method_params
    //         decoded_txn_data.1, // function name
    //         decoded_txn_data.2, // function id
    //         decoded_txn_data.3, // transaction receipt
    //         contract_address.clone().to_owned(),
    //         String::from(&contract_slug),
    //         &chain_id,
    //     )
    //     .await;

    println!("Added txn:{:?}", txn_hash);
        }
    prev_txn_hash = txn_hash;
    println!("//////////////////////////////////////////////////////////");
        }
    }
    }
    println!(
        "============================================================================="
    );

    Ok(())
}


fn is_interesting_method(method_of_interest:&HashSet<String>,method_name:&String)-> bool{
    if !method_of_interest.is_empty(){
        return method_of_interest.contains(method_name.as_str());
    }
    return true;
}
