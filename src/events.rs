use crate::{
    db::index::{db_log_store, db_meta_store},
    structs::{
        contracts::{ContractAbi, ContractMetaData},
        extract::{Db, Schema},
        log::Log,
        meta::MetaResult,
        transactions::{Transaction, TransactionEvent, TransactionIndexed}, 
    }, utils::meta::{utils_meta_indexed},
};
use ethers::{
    contract::ContractInstance,
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, Filter, ValueOrArray},
};
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
                    let transaction_struct: Transaction = Transaction {
                        txn_hash: log.transaction_hash.unwrap(),
                        block_hash: None,
                        block_number: None,
                        contract_address: None,
                        chain_id: None,
                        gas_used: None,
                        gas_price: None,
                        from: None,
                        to: None,
                    };

                    let transaction_indexed: TransactionIndexed = TransactionIndexed {
                        timestamp: None,
                        transaction: transaction_struct,
                        method: None,
                        event: Some(transaction_event),
                    };

                    let object: Option<MetaResult> = utils_meta_indexed(&schema, transaction_indexed).await;
                    meta_objects.push(object.unwrap());
                    // info!("\n\ntransaction_indexed -> {:?}\n\n", transaction_indexed);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            };
        }
        if !meta_objects.is_empty() {
            info!("Adding history_events meta_indexed into db...");
            let _ = db_meta_store(&db, &meta_objects).await;
        }
    }
    let logger: Log = Log {
        slug: schema.slug.to_string(),
        docsLength: meta_objects.len().to_string(),
        blockStart: schema.indexing.startBlock.to_string(),
        blockEnd: schema.indexing.endBlock.to_string(),
    };
    let _ = db_log_store(&db, logger).await;
}
