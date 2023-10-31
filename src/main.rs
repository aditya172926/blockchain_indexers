use env_logger::Env;
// use hex_literal::hex;
use ethcontract::contract::Instance;
use ethcontract::log::LogFilterBuilder;
use ethcontract::{prelude::*, transport, Topic};
use ethers::abi::{ethereum_types, Event, TopicFilter};
use ethers::providers::{Middleware, Provider};
use ethers::types::{BlockNumber, Filter, H160, H256, U64};
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
use std::sync::Arc;
use structs::contracts::ContractAbi;
use utils::db::utils_db;
use utils::index::utils_contract_instance;
use utils::reader;
use web3::transports::{http, Http};
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
    let schema: structs::extract::Schema =
        reader::utils_schema(String::from(config.slug.to_string()));
    let network_metadata: structs::networks::NetworkStruct =
        utils::networks::utils_network_data(schema.source[0].networkId).unwrap();
    let contract_result: (structs::contracts::ContractMetaData, ContractAbi) =
        utils::contracts::utils_contract_data(&config, &schema).await;

    let contract_metadata: structs::contracts::ContractMetaData = contract_result.0;
    // let contract_abi: structs::contracts::ContractAbi = contract_result.1;

    let transport: Http = Http::new(&network_metadata.network_rpc_url)?;
    let my_web3: Web3<Http> = Web3::new(transport);

    let contract_address_h160: ethers::types::H160 =
        contract_metadata.contract_address.parse().unwrap();
    let contract_address_historical: ethers::types::H160 = contract_metadata
        .contract_address_historical
        .parse()
        .unwrap();
    let read_abi_from_h160: H160 = contract_metadata.read_abi_from.parse().unwrap();

    let provider: Provider<ethers::providers::Http> =
        Provider::<ethers::providers::Http>::try_from(&network_metadata.network_rpc_url).unwrap();
    let client: Arc<Provider<ethers::providers::Http>> = Arc::new(provider);
    let contract = ethers::contract::Contract::new(contract_address_h160, c_abi, client);
    
    // let contract_instance: Instance<Http> = utils_contract_instance(
    //     my_web3.clone(),
    //     contract_abi.raw.clone(),
    //     contract_address_h160,
    // );

    let strblc = ethers::types::U64::from(18447805);
    let endblc = ethers::types::U64::from(18447807);

    //example of how to be create a topic
    // let mut topic = web3::ethabi::TopicFilter::default();
    // let topic0: ethcontract::TransactionHash =
    //     "0xb3d987963d01b2f68493b4bdb130988f157ea43070d4ad840fee0466ed9370d9"
    //         .parse()
    //         .unwrap();
    // 👆 this is for event: NameRegistered (string name, index_topic_1 bytes32 label, index_topic_2 address owner, uint256 baseCost, uint256 premium, uint256 expires)

    //  topic.topic0=Topic::OneOf(vec![topic0]);
    //  let  topic0=Topic::This(topic0);

    // let mut filter: ethcontract::log::LogFilterBuilder<ethcontract::Http> =
    //     LogFilterBuilder::new(my_web3)
    //         .from_block(BlockNumber::Number(strblc))
    //         .to_block(BlockNumber::Number(endblc))
    //         .address(vec![addr])
    //         .block_page_size(100)
    //         .limit(10)
    //         .poll_interval(core::time::Duration::new(1, 0))
    //         .topic0(Topic::This(topic0));

    let topic: ethers::types::H256 =
        "0x69e37f151eb98a09618ddaa80c8cfaf1ce5996867c489f45b555b412271ebf27".parse()?;
    let filter2: ethers::types::Filter = Filter::new()
        .address(ethers::types::ValueOrArray::Array(vec![
            contract_address_historical,
        ]))
        .from_block(BlockNumber::Number(strblc))
        .to_block(BlockNumber::Number(endblc))
        .topic0(ethers::types::ValueOrArray::Value(topic));

    let abi = contract_result.1.string;
    let c_abi: ethers::core::abi::Abi = serde_json::from_str(&abi).unwrap();

    // let contract_event = contract.event::<>().filter(filter2);
    let logs = client.get_logs(&filter2).await?;
    for log in logs {
        println!(
            "\n\nTransaction hash -> {:?} \n\nLog topics -> {:?} \n Log data -> {:?}\n\n",
            log.transaction_hash,
            log.topics,
            log.data
        );
        match contract.decode_event_raw("NameRegistered", log.topics, log.data) {
            Ok(inputs) => {
                println!("\n\ndecode_log -> {:?}\n\n", inputs);
            },
            Err(error) => {
                println!("{:?}", error);
            }
        };
        let contract_event_instance = &contract_result
            .1
            .stat
            .event("NameRegistered")
            .unwrap()
            .inputs;
        println!("{:?}", contract_event_instance);
    }

    // let contract_event = ethers::contract::Event{filter: filter2};

    // we have to create topc0,topic1,topic2 and topic3 to make it specific to events we want
    // LogFilterBuilder: https://docs.rs/ethcontract/latest/ethcontract/log/struct.LogFilterBuilder.html
    //Topic enum: https://docs.rs/ethabi/18.0.0/ethabi/enum.Topic.html
    // example for topics: https://ethereum.stackexchange.com/questions/132794/erc20-event-listener-in-rust-programming

    // let logs = filter.past_logs().await.unwrap();
    // let event: ethers::abi::Event = contract_abi
    //     .stat
    //     .event("NameRegistered").cloned()
    //     .expect("couldn't create event object, not in abi");

    // let decoded_events = &event.inputs;
    // println!("\n\n{:?}\n\n", decoded_events);

    // if logs.len() == 0 {
    //     println!("Empty");
    // } else {
    //     for log in logs {
    //         // let decoded_log = event.parse_log(log).unwrap();

    //         // let raw = RawLog{topics: log.topics, data: Vec::from(log.data)};
    //         let decoded_log = RawLog::from(log);
    //         // let decoded_raw = RawLog::decode(decoded_log);

    //         // println!("Raw log -> {:?}", decoded_log.decode(event));
    //         println!("\n\n-----------------------------------------------NEXT!-----------------------------------------------\n\n");
    //     }
    // }

    exit(1);

    // let client=Provider::<ethers::providers::Http>::try_from("https://eth-mainnet.g.alchemy.com/v2/wiflVw_lj8Lx6x6n0GYWEMhQgMqnFW8x").unwrap();

    // let abi=contract_result.1.string;
    // let c_abi: ethers::core::abi::Abi = serde_json::from_str(&abi).unwrap();
    // let contract =ethers::contract::Contract::new(addr,c_abi,Arc::new(client));
    // // let txn_event=contract.event_for_name::<ValueChanged>("Transfer").unwrap();

    // let logs = contract
    // .event()
    // .from_block(strblc).to_block(endblc)
    // .query()
    // .await;

    // println!("{:?}", logs);

    //     let ev = contract.event::<ValueChanged>().from_block(17125818).to_block(171265819);

    //     let mut log_stream=Box::pin(ev.stream().await.unwrap());
    //     // println!("The stream:{:?}",);

    // //     while let Some(Ok(approval)) = log_stream.next().await {
    // //         println!("Event caught:{:?}",approval);
    // //    }
    // loop {

    //    match logs.next().await {

    //             Ok(ev) => {println!("New event: {:?}", ev)},
    //             Err(e) => {println!("Error: {:?}", e)},
    //     }
    // }

    // let my_event = contract.event::<ValueChanged>();
    // let watcher=my_event::watcher().from_block(5).to_block(10);
    // let stream=

    // if &config.mode == "HISTORY_TXN" {
    //     let _ = transactions::get_history(
    //         &db,
    //         &schema,
    //         &contract_metadata,
    //         &network_metadata,
    //         &contract_abi,
    //     )
    //     .await;
    // } else if &config.mode == "LIVE_TXN" {
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
