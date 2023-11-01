use crate::structs::{
    contracts::{ContractAbi, ContractMetaData},
    extract::{Config, Schema},
    networks::NetworkStruct,
};
use env_logger::Env;
use ethers::contract::{Contract, ContractInstance};
use ethers::providers::{Http, Provider};
use log::{debug, error, info, warn};
use std::error::Error;
use std::string::String;
use std::sync::Arc;
use utils::db::utils_db;
use utils::reader;

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
    // pub(crate) mod poap_ethereum;
    pub(crate) mod ud_ethereum;
}

mod helpers {
    pub(crate) mod erc721;
    pub(crate) mod url;
}
mod events;
use ethers::contract::{self, EthEvent};

#[derive(Debug, Clone, EthEvent, Copy)]
pub struct Transfer {
    #[ethevent(indexed)]
    pub from: ethers::types::Address,
    #[ethevent(indexed)]
    pub to: ethers::types::Address,
    pub tokenId: ethers::types::U256,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let f = std::fs::File::open(String::from("config/index.yml")).expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    let db = utils_db(config.env.to_string()).await;

    let schema: Schema = reader::utils_schema(String::from(config.slug.to_string()));
    let network_metadata: NetworkStruct =
        utils::networks::utils_network_data(schema.source[0].networkId).unwrap();

    let provider: Provider<Http> =
        Provider::<Http>::try_from(&network_metadata.network_rpc_url).unwrap();
    let client: Arc<Provider<Http>> = Arc::new(provider);

    let contract_result: (ContractMetaData, ContractAbi) =
        utils::contracts::utils_contract_data(&config, &schema).await;

    let contract_metadata: ContractMetaData = contract_result.0;
    let contract_abi = contract_result.1;

    let contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>> = Contract::new(
        contract_metadata.contract_address_H160,
        contract_abi.stat.clone(),
        client.clone(),
    );

    if &config.mode == "HISTORY_EVENTS" {
        let _ = events::get_history_events(
            &db,
            &client,
            &schema,
            &contract_metadata,
            &contract_abi,
            contract,
        )
        .await;
    } else if &config.mode == "HISTORY_TXN" {
        let _ = transactions::get_history_txns(
            &db,
            &schema,
            &contract_metadata,
            &network_metadata,
            &contract_abi,
        )
        .await;
    }
    // else if &config.mode == "LIVE_TXN" {
    //     let _ = transactions::get_txns(
    //         &db,
    //         &schema,
    //         &contract_abi,
    //         &contract_instance,
    //         contract_metadata,
    //         &network_metadata,
    //     )
    //     .await;
    // }

    Ok(())
}
