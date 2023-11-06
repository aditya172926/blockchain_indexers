use crate::{
    db::index::{db_log_store, db_meta_store},
    events,
    handlers::poap_ethereum::handler::handler_transfer_poap,
    structs::{
        contracts::{ContractAbi, ContractIndexed, ContractMetaData},
        extract::{Db, Schema},
        log::Log,
        meta::MetaResult,
        transactions::{Transaction, TransactionEvent, TransactionIndexed},
    },
    utils::{handler::handler_data_from_event, meta::utils_meta_indexed},
};
use ethers::{
    abi::{token, Hash},
    contract::ContractInstance,
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, Bytes, Filter, ValueOrArray, H256},
};
use log::{debug, error, info, warn};
use std::{collections::HashMap, fmt::format, sync::Arc};

pub async fn get_history_events(
    db: &Db,
    client: &Arc<Provider<Http>>,
    schema: &Schema,
    contracts: &mut Vec<ContractIndexed>,
) -> () {
    let mut txn_objects: HashMap<String, Vec<TransactionEvent>> = HashMap::new();

    let mut meta_objects: Vec<MetaResult> = Vec::new();
    let mut topics: Vec<H256> = vec![];
    let mut addresses: Vec<ethers::types::H160> = vec![];
    for c in contracts.clone() {
        topics.append(&mut c.data.events_of_interest.topics.clone());
        addresses.push(c.data.contract_address_historical_H160.clone());
    }
    // info!("topis : {:?}", topics);

    let event_filter: Filter = Filter::new()
        .address(ValueOrArray::Array(addresses))
        .from_block(BlockNumber::Number(schema.indexing.startBlock.into()))
        .to_block(BlockNumber::Number(schema.indexing.endBlock.into()))
        .topic0(ValueOrArray::Array(topics));

    let logs = client.get_logs(&event_filter).await.unwrap();
    for log in logs {
        let topic0 = log.topics[0] as Hash;
        for contract in &mut *contracts {
            if contract.data.events_of_interest.topics.contains(&topic0) {
                match contract.instance.decode_event_raw(
                    &contract.data.events_of_interest.map[&log.topics[0]],
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(inputs) => {
                        let txn_hash: String = format!("0x{:x}", &log.transaction_hash.unwrap());

                        let mut txn_event: TransactionEvent = TransactionEvent {
                            topic0: topic0,
                            name: contract.data.events_of_interest.map[&topic0].clone(),
                            params: inputs.clone(),
                            data: None,
                        };
                        // info!("txn event {:?} ", txn_event);
                        let txn_data = handler_data_from_event(schema, contract, txn_event.clone())
                            .await
                            .unwrap();
                        txn_event.data = Some(txn_data);

                        match txn_objects.get(&txn_hash) {
                            Some(events) => {
                                let mut events_new = events.clone();
                                events_new.push(txn_event.clone());
                                txn_objects.insert(txn_hash, events_new);
                            }
                            None => {
                                txn_objects.insert(txn_hash, vec![txn_event.clone()]);
                            }
                        }
                        // info!("txn event is : {:?}", txn_event);
                    }
                    Err(error) => {
                        println!("{:?}", error);
                    }
                };
            }
        }
    }
    for (key, val) in txn_objects.iter() {
        let transaction_struct: Transaction = Transaction {
            txn_hash: Some(key.to_string()),
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
            events: Some(val.to_vec()),
        };

        // info!(
        //     "transaction indexed for a txn hash {:?}",
        //     transaction_indexed
        // );

        let object: Option<MetaResult> = utils_meta_indexed(&schema, transaction_indexed).await;
        meta_objects.push(object.unwrap());
    }

    if !meta_objects.is_empty() {
        info!(
            "Adding history_events meta_indexed into db...\n{:?}",
            meta_objects
        );
        // let _ = db_meta_store(&db, &meta_objects).await;
    }

    // let logger: Log = Log {
    //     slug: schema.slug.to_string(),
    //     docsLength: meta_objects.len().to_string(),
    //     blockStart: schema.indexing.startBlock.to_string(),
    //     blockEnd: schema.indexing.endBlock.to_string(),
    // };
    // let _ = db_log_store(&db, logger).await;
}
