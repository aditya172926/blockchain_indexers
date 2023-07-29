use ethers::core::types::Chain;
use ethers::etherscan::account::TxListParams;
use ethers::etherscan::Client;
use ethers::prelude::*;
use ethers::{
    prelude::account::Sort,
    providers::Provider
};

pub async fn get_history(
    contract_address: &str,
    start_block: u64,
    end_block: u64,
    rpc_url: &str,
    api_key: &str
) -> eyre::Result<()> {
    println!("CHECKING HISTORY...");
    let _provider = Provider::try_from(rpc_url)?;

    //chain was to be generalized
    let client = Client::builder()
        .with_api_key(api_key)
        .chain(Chain::Mainnet)
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
        let txn_hash: String = match txn.hash {
            account::GenesisOption::None => String::from("NA"),
            account::GenesisOption::Genesis => String::from("0x00"),
            account::GenesisOption::Some(i) => i.to_string(),
        };
        let function_name: String = match txn.function_name {
            Some(i) => i.to_string(),
            None => String::from("NA"),
        };

        println!("Sender:{:?},Recipient:{:?}, Value:{:?}, Contract used:{:?}, Block Number:{:?}, Function Used:{}",from,to,value,contract_used,block_number,function_name);
    }

    Ok(())
}
