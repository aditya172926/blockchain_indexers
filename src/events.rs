use ethers::types::{Filter, ValueOrArray, BlockNumber, H256};
use ethers::providers::{Provider, Http, Middleware};
use ethers::contract::ContractInstance;
use std::sync::Arc;
use crate::structs::{contracts::{ContractMetaData, ContractAbi}, extract::{Schema, Db}};
use log::{debug, error, info, warn};

pub async fn get_history_events(
    db: &Db,
    client: &Arc<Provider<Http>>,
    schema: &Schema,
    contract_metadata: &ContractMetaData,
    contract_abi: &ContractAbi,
    contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
) -> () {
    
    for event in &contract_metadata.events_of_interest {
        info!("Event from schema {:?}", event);
        let event_filter: Filter = Filter::new()
            .address(ValueOrArray::Array(vec![contract_metadata.contract_address_historical_H160]))
            .from_block(BlockNumber::Number(schema.indexing.startBlock.into()))
            .to_block(BlockNumber::Number(schema.indexing.endBlock.into()))
            .topic0(ValueOrArray::Array(vec![event.topic0]));

        let logs = client.get_logs(&event_filter).await.unwrap();
        for log in logs {
            println!(
                "\n\nLog -> {:?} \n\nLog topics -> {:?} \n Log data -> {:?}\n\n",
                log, log.topics, log.data
            );
            match contract_instance.decode_event_raw(&event.name, log.topics, log.data) {
                Ok(inputs) => {
                    println!("\n\ndecode_log -> {:?}\n\n", inputs);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            };
        }
        let contract_event_params = &contract_abi.stat.event(&event.name).unwrap().inputs;
        println!("{:?}", contract_event_params);
    }
    
}