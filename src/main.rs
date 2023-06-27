#[allow(unused)]

use sqlx::postgres::{PgPoolOptions,PgRow};
use sqlx::{FromRow,Row};
use ethers::{
    providers::{Provider, Http, Middleware},
    types::{Address, Log, BlockNumber, Filter}, prelude::account::Sort,
};
use ethers::prelude::*;
use ethers::core::types::Chain;
use ethers::etherscan::Client;
use ethers::etherscan::account::TxListParams;

pub struct EventSchema{
    id:i64,
    sender:String,
    recipient:String,
    contract_used:String,
    value:String,
    block_number:String,
    txn_hash:String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {

    println!("running");
    
    let rpc_url = "https://lingering-delicate-choice.discover.quiknode.pro/68f9e3726efe97ee2b6a7c8417f6f5d12ab713c6/";
    let _provider = Provider::try_from(rpc_url)?;

    let client = Client::builder().with_api_key("ER9VKT8AXAI2WTPSCRNANN69W67V7PRU59").chain(Chain::Mainnet).unwrap().build().unwrap();

//Calling Functions
    get_transactions(&client).await?;
    
    println!("all okay");
    Ok(())
}


async fn get_transactions(client:&Client)-> eyre::Result<()>{
    //Array to hold addresses
    let accounts = vec![
        "0x59077aC13294c154180412212241641704E94ed3"
    //    "0x4cBC53136BdC87907442Cd8b2AdF7B0A738C7bc5",
    //    "0xC18360217D8F7Ab5e7c516566761Ea12Ce7F9D72"
    // "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"
    //  "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D"

  ];


    for acc in accounts{
        println!("for account: {} ",acc);

        
        let paras= TxListParams{
            
            start_block: 16637965,
            end_block: 17557269,
            page: 0,
            offset: 0,
            sort: Sort::Asc,
            
        };


//Fetching all transactions
        let txns=client.get_transactions(&acc.parse().unwrap(),Some(paras)).await.unwrap();
        // println!("{:?}",&txns);

//Creating loop to iterate over all transactions
        for txn in txns{
            let from=txn.from;
                let mut s_from: String=String::from("s");
                let mut s_to:String=String::from("s");
                let mut s_contract_used:String=String::from("s");
                let mut s_txn_hash:String=String::from("s");
                

//Formatting everything to String                
                if let account::GenesisOption::Some(i)=from{
                    s_from=i.to_string();
                }
                let to=txn.to;
                if let Some(i)=to{
                    s_to=i.to_string();
                }
                let value:String=txn.value.to_string();
                let contract_used=txn.contract_address;
                
                let s_contract_used:String=match contract_used{
                    Some(i)=>i.to_string(),
                    None=>String::from("NA"),
                };
                let block_number=txn.block_number.to_string();
                // let txn_hash:String=match txn.hash{
                //     account::GenesisOption::None => String::from("NA"),
                //     account::GenesisOption::Genesis => String::from("0x00"),
                //     account::GenesisOption::Some(i) => i.to_string(),
                // };
                let function_name:String=match txn.function_name{
                    Some(i)=>i.to_string(),
                    None=>String::from("NA"),
                };
              

                println!("Sender:{:?},Recipient:{:?}, Value:{:?}, Contract used:{:?}, Block Number:{:?}, Function Used:{}",from,to,value,contract_used,block_number,function_name);

                add_to_db(s_from,s_to,value,s_contract_used,block_number,function_name).await?;

        }
}
Ok(())
}



async fn add_to_db(sender:String,recipient:String,value:String,contract_used:String,block_no:String,function_name:String) -> Result<(),sqlx::Error>{

    //Create instance
    let pool=PgPoolOptions::new().max_connections(5).connect("postgres://postgres:grespost51@localhost/test").await?;
    
    //Create table
            sqlx::query(
                r#"
        CREATE TABLE IF NOT EXISTS transaction (
        id bigserial,
        sender text,
        recipient text,
        value text,
        contract_used text,
        block_number text,
        function_name text
        );"#,
            )
            .execute(&pool)
            .await?;
    
    //insert new event
    
    
        let _row: (i64,) = sqlx::query_as(
            r#"
            INSERT INTO transaction (sender,recipient,value,contract_used,block_number,function_name)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#
        )
        .bind(sender)
        .bind(recipient)
        .bind(value)
        .bind(contract_used)
        .bind(block_no)
        .bind(function_name)
        .fetch_one(&pool)
        .await?;
    
        Ok(())
    }






//Can return following data:

// is_error: "0", 
// block_number: Number(17265892), 
// time_stamp: "1684162391", 
// hash: Some(0x1f3fac623cd2a6527ef704893af64516ec99887a64ac26f46b916fb48f6c1848), 
// nonce: Some(3), 
// block_hash: Some(65165998095695600221651197282961762370363245438279807374813381549706017278872), transaction_index: Some(144), 
// from: Some(0x59077ac13294c154180412212241641704e94ed3), 
// to: Some(0xdac17f958d2ee523a2206206994597c13d831ec7), 
// value: 0, 
// gas: 69146, 
// gas_price: Some(71494883816), 
// tx_receipt_status: "1", 
// input: Bytes(0xa9059cbb00000000000000000000000098cb2cdb5a4433764467a47c888f6ad98774c12100000000000000000000000000000000000000000000000000000000000f4240), 
// contract_address: None, 
// gas_used: 46097, 
// cumulative_gas_used: 13811088, 
// confirmations: 196420, 
// method_id: Some(0xa9059cbb), 
// function_name: Some("transfer(address _to, uint256 _value)
