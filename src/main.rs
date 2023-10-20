use ethcontract::contract::Instance;
use ethcontract::prelude::*;
use ethers::providers::Provider;
use ethers::types::{H256, Filter};
use futures::join;
use futures::stream::StreamExt;
use hex::ToHex;
use mongodb::bson::document::ValueAccessError;
use mongodb::bson::Document;
use structs::contracts::ContractAbi;
use std::collections::HashSet;
use std::string::String;
use std::{error::Error, str::FromStr};
use tokio::time::{sleep, Duration};
use web3::transports::Http;
use web3::Web3;
// use crate::structs::{ContractData, MethodParam, Transaction, TransactionIndexed};
use chrono::prelude::*;
use mongodb::{
    bson::{doc, to_bson, Bson},
    options::ClientOptions,
    Client, 
};
use log::{debug, error, info, warn};
use env_logger::Env;

use crate::handlers::ens_ethereum::handler;

// modules
mod db{
    pub(crate) mod index;
}
mod utils{
    pub(crate) mod index;
    pub(crate) mod transactions;
    pub(crate) mod networks;
    pub(crate) mod contracts;
}
mod transactions;
mod middleware;
mod structs{
    pub(crate) mod index;
    pub(crate) mod transactions;
    pub(crate) mod networks;
    pub(crate) mod contracts;
    pub(crate) mod meta;
}

mod abstractor;

mod handlers{
    pub(crate) mod ens_ethereum;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let contract_result: (structs::contracts::ContractMetaData,ContractAbi ) =
        utils::contracts::utils_contract_data("lens_profile_polygon").await;

    let contract_metadata: structs::contracts::ContractMetaData = contract_result.0;
    let contract_abi: structs::contracts::ContractAbi = contract_result.1;
    
    let network_metadata: structs::networks::NetworkStruct =
       utils::networks::utils_network_data(&contract_metadata.chain_id).unwrap();
    
    let transport: Http = Http::new(&network_metadata.network_rpc_url)?;
    let web3: Web3<Http> = Web3::new(transport);

    let contract_address_h160: H160 = contract_metadata.contract_address.parse().unwrap();
    let read_abi_from_h160:H160 = contract_metadata.read_abi_from.parse().unwrap();
    // println!("\n\n\n\n\n read_abi_from {} \n\n\n\n\n", read_abi_from.to_string());
    let contract_instance: Instance<Http> =
        Instance::at(web3, contract_abi.raw, contract_address_h160);
        
    // let contract_address_string = format!("{:020x}", contract_address_h160);
    // let read_abi_from_string = format!("{:020x}",read_abi_from_h160);
    // let initial = String::from("0x");
    // let s_contract_address = format!("{}{}", initial, contract_address_string);
    // let s_read_abi_from = format!("{}{}",initial,read_abi_from_string);

    let start_block: u64 = 48888559;
    let end_block: u64 = 48894632;
    
    let _ = transactions::get_history(
        &contract_metadata.read_abi_from,
        &contract_abi.string,
        start_block,
        end_block,
        contract_metadata.chain_id,
        contract_metadata.contract_slug,
        &network_metadata.network_rpc_url,
        &network_metadata.network_api_key,
        contract_metadata.methods,
        contract_metadata.method_of_interest,
        network_metadata.network_rpc_url.clone()
    )
    .await;

    // let_ = transactions::get_txns(
    //     &contract_abi.string,
    //     &contract_instance,
    //     contract_metadata.function_of_interest,
    //     s_contract_address,
    //     contract_metadata.chain_id,
    //     contract_metadata.contract_slug,
    //     network_metadata.network_rpc_url,
    //     network_metadata.start_block_number,
    //     contract_metadata.method_of_interest,
    //     contract_metadata.methods,
    // )
    // .await;

    // let _ = get_events(contract_instance, 17630615).await;

    Ok(())
}