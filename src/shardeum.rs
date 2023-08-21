//ethers_providers
use ethers_providers::Middleware;
use tokio::time::{sleep, Duration};


pub async fn get_shardeum_data(contract_address:String){
    println!("here");
    let rpc_shardeum_https = "https://sphinx.shardeum.org/";

    let provider = ethers_providers::Provider::<ethers_providers::Http>::try_from(rpc_shardeum_https).expect("could not instantiate HTTP Provider");

    let chainid = provider.get_chainid().await.unwrap();
    println!("Got chainid: {}", chainid);

    // loop{
    //     let current_block_number = provider.get_block_number().await.unwrap();

    //     let current_cycle_number = current_block_number/10;
    //     // println!("current_cycle_number: {:?}", current_cycle_number);

        let base_url = String::from("https://explorer-sphinx.shardeum.org/api/transaction?startCycle=");
        // let cycle_number=58161;
        let cycle_number=58161;

    //     let transaction_count : u64 = get_transaction_count(startCycle.clone(), base_url.clone()).await;    
    //     println!("transaction_count: {:#?}", transaction_count);
        
    //     read_json_loop(startCycle.clone(), base_url.clone(), transaction_count).await;
        
    //     sleep(Duration::from_millis(30000)).await; // Sleep for 30 seconds.
    // }

    let transaction_count : u64 = get_transaction_count(cycle_number.clone(), base_url.clone(),&contract_address).await;    
    println!("transaction_count: {:#?}", transaction_count);
     
    read_json_loop(cycle_number.clone(), base_url.clone(), transaction_count,contract_address).await;


}

async fn get_transaction_count(cycle_number: u64, base_url: String,contract_address:&str) -> u64   {

    let get_request_url = 
        base_url +
        &cycle_number.to_string() +
        "&endCycle=" +
        &cycle_number.to_string()+
        "&address="+contract_address;
    println!("getRequestUrl: {:#?}", get_request_url);

    let new_todo: serde_json::Value = reqwest::Client::new()
        .get(get_request_url)
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();

    println!("JSON raw response: {:#?}", new_todo);
    println!("{:#?}", new_todo["success"]);
    println!("{:#?}", new_todo["totalTransactions"]);
    println!("{:#?}", new_todo["totalTransactions"].as_u64().unwrap());

    return new_todo["totalTransactions"].as_u64().unwrap();

}
async fn read_json_loop(cycle_number: u64, base_url: String, total_transactions: u64,contract_address:String) {

    println!("checking");

    let mut total : i64 = total_transactions as i64; //Convert value to be signed so we do not have an underflow error when the value become negative.
    let mut page_index = 1;
 
    while total > 0 {
 
         let get_request_url = 
             base_url.clone() +
             &cycle_number.to_string() +
             "&endCycle=" +
             &cycle_number.to_string() + 
             "&address="+&contract_address+
             "&page=" + 
             &page_index.to_string();
         println!("the second one: {:#?}", get_request_url);
 
         let new_todo: serde_json::Value = reqwest::Client::new()
             .get(get_request_url)
             .send()
             .await.unwrap()
             .json()
             .await.unwrap();
            let transactions_array=new_todo["transactions"].as_array().unwrap();
        //  println!("JSON raw response: {:#?}", new_todo);
        for i in transactions_array.iter(){
            println!("//////////////////////////NEXT TRANSACTION:");
            println!("txId:{:#?}",i["txId"]);
            println!("timestamp:{:?}",i["timestamp"]);
            println!("from:{:?}",i["wrappedEVMAccount"]["readableReceipt"]["from"]);
            println!("to:{:?}",i["wrappedEVMAccount"]["readableReceipt"]["to"]);
            println!("gasUsed:{:?}",i["wrappedEVMAccount"]["readableReceipt"]["gasUsed"]);
            println!("blockHash:{:?}",i["wrappedEVMAccount"]["readableReceipt"]["blockHash"]);
            println!("blockNumber:{:?}",i["wrappedEVMAccount"]["readableReceipt"]["blockNumber"]);
            println!("value:{:?}",i["wrappedEVMAccount"]["readableReceipt"]["value"]);
        }
 
         total -= 10;
         page_index += 1;
     }   
     
 }