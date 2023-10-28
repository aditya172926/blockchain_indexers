use env_logger::Env;
use ethcontract::contract::Instance;
use ethcontract::prelude::*;
use ethers::providers::Provider;
use ethers::types::{Filter, H256};
use futures::join;
use futures::stream::StreamExt;
use hex::ToHex;
use log::{debug, error, info, warn};
use mongodb::bson::document::ValueAccessError;
use mongodb::bson::Document;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::process::exit;
use std::string::String;
use structs::contracts::ContractAbi;
use utils::db::utils_db;
use utils::index::utils_contract_instance;
use utils::reader;
use web3::transports::Http;
use web3::Web3;

use crate::structs::extract::Config;
use crate::structs::log::Log;

// use crate::handlers::ens_ethereum::handler_ens;

// modules
mod db {
    pub(crate) mod index;
}
mod utils {
    pub(crate) mod contracts;
    pub(crate) mod db;
    pub(crate) mod index;
    pub(crate) mod meta;
    pub(crate) mod networks;
    pub(crate) mod reader;
    pub(crate) mod transactions;
}
mod transactions;
mod structs {
    pub(crate) mod contracts;
    pub(crate) mod extract;
    pub(crate) mod index;
    pub(crate) mod log;
    pub(crate) mod meta;
    pub(crate) mod networks;
    pub(crate) mod transactions;
}

mod abstractor;

mod handlers {
    pub(crate) mod ens_ethereum;
    pub(crate) mod lens_post;
    pub(crate) mod lens_profile_polygon;
    pub(crate) mod poap_ethereum;
}

mod helpers {
    pub(crate) mod erc721;
    pub(crate) mod url;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let f = std::fs::File::open(String::from("config/index.yml")).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    info!("after parsing index json {} ", config.env.to_string());

    let db = utils_db(config.env.to_string()).await;

    let schema: structs::extract::Schema =
        reader::utils_schema(String::from(config.slug.to_string()));

    let network_metadata: structs::networks::NetworkStruct =
        utils::networks::utils_network_data(schema.source[0].networkId).unwrap();

    let contract_result: (structs::contracts::ContractMetaData, ContractAbi) =
        utils::contracts::utils_contract_data(&config,&schema).await;

    let contract_metadata: structs::contracts::ContractMetaData = contract_result.0;
    let contract_abi: structs::contracts::ContractAbi = contract_result.1;

    let transport: Http = Http::new(&network_metadata.network_rpc_url)?;
    let web3: Web3<Http> = Web3::new(transport);

    let contract_address_h160: H160 = contract_metadata.contract_address.parse().unwrap();
    let read_abi_from_h160: H160 = contract_metadata.read_abi_from.parse().unwrap();
    let contract_instance: Instance<Http> =
        utils_contract_instance(web3, contract_abi.raw.clone(), contract_address_h160);

    if &config.mode == "HISTORY_TXN" {
        let _ = transactions::get_history(
            &db,
            &schema,
            &contract_metadata,
            &network_metadata,
            &contract_abi,
        )
        .await;
    } else if &config.mode == "LIVE_TXN" {
        let _ = transactions::get_txns(
            &db,
            &schema,
            &contract_abi,
            &contract_instance,
            contract_metadata,
            &network_metadata,
        )
        .await;
    }

    Ok(())
}
