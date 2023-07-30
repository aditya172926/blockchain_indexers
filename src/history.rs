use ethers::core::types::Chain;
use ethers::etherscan::account::TxListParams;
use ethers::etherscan::Client;
use ethers::prelude::*;
use ethers::{
    prelude::account::Sort,
    providers::Provider
};
use mongodb::bson::Document;

use crate::{structs, transactions,db};

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
) -> eyre::Result<()> {
    println!("CHECKING HISTORY...");
    let _provider = Provider::try_from(rpc_url)?;

    //chain was to be generalized *IMPORTANT:CHANGE CHAIN NAME ACCORDING TO CONTRACT*
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
        let value = txn.value.to_string();
        let contract_used = txn.contract_address;

        let s_contract_used: String = match contract_used {
            Some(i) => i.to_string(),
            None => String::from("NA"),
        };
        let block_number = txn.block_number.to_string();
        let function_name: String = match txn.function_name {
            Some(i) => i.to_string(),
            None => String::from("NA"),
        };
        let t_hash: String = match txn.hash {
            account::GenesisOption::None => String::from("NA"),
            account::GenesisOption::Genesis => String::from("0x00"),
            account::GenesisOption::Some(i) => i.to_string(),
        };

        // println!("Sender:{:?},Recipient:{:?}, Value:{:?}, Contract used:{:?}, Block Number:{:?}, Function Used:{}",from,to,value,contract_used,block_number,function_name);


        let txn_hash=txn.hash.value().unwrap().to_owned();

        let mut decoded_txn_data: (
            Vec<structs::MethodParam>,         // method params array
            String,                            // function name
            String,                            // transaction hash
            ethers::types::TransactionReceipt, // transaction receipt
        ) = transactions::get_transaction_data(
            contract_abi,
            txn_hash,
            &rpc_url,
            &methods,
        )
        .await;

    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("{:?}", decoded_txn_data);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

    let _ = db::save_history_to_db(
        decoded_txn_data.0, //method_params
        decoded_txn_data.1, // function name
        decoded_txn_data.2, // function id
        decoded_txn_data.3, // transaction receipt
        contract_address.clone().to_owned(),
        String::from(&contract_slug),
        &chain_id,
    )
    .await;
    println!("Added txn:{:?}", txn_hash);
    }

    Ok(())
}
