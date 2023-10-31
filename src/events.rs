use ethers::types::{Filter, ValueOrArray, BlockNumber, H256};
use ethers::providers::{Provider, Http};
use ethers::contract::ContractInstance;
use std::sync::Arc;
use std::error::Error;
use crate::structs::{contracts::ContractMetaData, extract::{Schema, Db}};

pub async fn get_history_events(
    db: &Db,
    schema: &Schema,
    contract_metadata: &ContractMetaData,
    contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
) -> eyre::Result<()> {
    
    for event: ContractEvent in contract_metadata.events_of_interest {

        let event_filter: Filter = Filter::new()
            .address(ValueOrArray::Array(contract_metadata.contract_address_historical_H160))
            .from_block(BlockNumber::Number(schema.indexing.startBlock))
            .to_block(BlockNumber::Number(schema.indexing.endBlock))
            .topic0(ValueOrArray::Value(event.topic0));

        let logs = client.get_logs(&event_filter).await?;
        for log in logs {
            println!(
                "\n\nTransaction hash -> {:?} \n\nLog topics -> {:?} \n Log data -> {:?}\n\n",
                log.transaction_hash, log.topics, log.data
            );
            match contract_instance.decode_event_raw(event.name, log.topics, log.data) {
                Ok(inputs) => {
                    println!("\n\ndecode_log -> {:?}\n\n", inputs);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            };
            let contract_event_params = &contract_result
                .1
                .stat
                .event(event.name)
                .unwrap()
                .inputs;
            println!("{:?}", contract_event_params);
        }
    }

    Ok(());
    
}