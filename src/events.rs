use crate::structs::{
    contracts::{ContractAbi, ContractMetaData},
    extract::{Db, Schema},
    transactions::TransactionEvent,
    meta::MetaResult
};
use ethers::contract::ContractInstance;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{BlockNumber, Filter, ValueOrArray, H256};
use log::{debug, error, info, warn};
use std::sync::Arc;

pub async fn get_history_events(
    db: &Db,
    client: &Arc<Provider<Http>>,
    schema: &Schema,
    contract_metadata: &ContractMetaData,
    contract_abi: &ContractAbi,
    contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
) -> () {
    let mut meta_objects: Vec<MetaResult> = Vec::new();

    for event in &contract_metadata.events_of_interest {
        info!("Event from schema {:?}", event);
        let event_filter: Filter = Filter::new()
            .address(ValueOrArray::Array(vec![
                contract_metadata.contract_address_historical_H160,
            ]))
            .from_block(BlockNumber::Number(schema.indexing.startBlock.into()))
            .to_block(BlockNumber::Number(schema.indexing.endBlock.into()))
            .topic0(ValueOrArray::Array(vec![event.topic0]));

        let logs = client.get_logs(&event_filter).await.unwrap();
        for log in logs {
            info!(
                "\n\nTransaction hash -> {:?} \n\nLog topics -> {:?} \n Log data -> {:?}\n\n",
                log.transaction_hash, log.topics, log.data
            );
            match contract_instance.decode_event_raw(&event.name, log.topics, log.data) {
                Ok(inputs) => {
                    let transaction_event: TransactionEvent = TransactionEvent {
                        topic0: event.topic0,
                        name: event.name.clone(),
                        params: inputs,
                    };
                    info!("\n\ntransaction_event -> {:?}\n\n", transaction_event);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            };
        }
    }
}
