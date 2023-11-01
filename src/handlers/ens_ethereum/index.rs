use ethers::types::H160;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::process::exit;

use crate::handlers::ens_ethereum::handler;
use crate::structs::extract::Schema;
use crate::structs::meta::{self, Meta, MetaIndexed, MetaResult};
use crate::structs::transactions::{TransactionEvent, TransactionIndexed};

pub async fn handler_events_ens(
    transaction_event: &TransactionEvent,
    schema: &Schema,
) -> Option<MetaResult> {
    if transaction_event.name.to_string() == "NameRegistered" {
        return handler::handler_event_register_ens_by_controller(transaction_event, schema).await;
    } else if transaction_event.name.to_string() == "NameRenewed" {
        return handler::handler_event_renew_ens_by_controller(transaction_event, schema).await;
    } else if transaction_event.name.to_string() == "Transfer" {
        return handler::handler_event_transfer_ens_by_base(transaction_event.clone(), schema)
            .await;
    } else {
        return None;
    }
}

pub async fn handler_ens(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    match &transaction_indexed.method {
        Some(transaction_indexed_method) => {
            if transaction_indexed_method.name == "register"
                || transaction_indexed_method.name == "registerOnly"
            {
                return handler::handler_txn_register_ens(transaction_indexed, schema).await;
            } else if transaction_indexed_method.name == "renew" {
                return handler::handler_txn_renew_ens(
                    transaction_indexed,
                    transaction_indexed_method,
                    schema,
                )
                .await;
            } else if transaction_indexed_method.name == "reclaim" {
                return handler::handler_txn_reclaim_ens(
                    transaction_indexed,
                    transaction_indexed_method,
                    schema,
                )
                .await;
            } else {
                return None;
            }
        }

        None => {
            let mut meta_raw: HashMap<String, String> = HashMap::new();

            for event in transaction_indexed.events.as_ref().unwrap() {
                if format!("0x{:x}", event.topic0)
                    == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                {
                    //source[1].Transfer (index_topic_1 address from, index_topic_2 address to, index_topic_3 uint256 tokenId)
                    meta_raw.insert("from".to_string(), event.params[0].to_string());
                    meta_raw.insert("to".to_string(), event.params[1].to_string());
                    meta_raw.insert("tokenId".to_string(), event.params[2].to_string());
                } else if format!("0x{:x}", event.topic0)
                    == "0xb3d987963d01b2f68493b4bdb130988f157ea43070d4ad840fee0466ed9370d9"
                {
                    //source[1].nameregistered
                    meta_raw.insert("tokenId".to_string(), event.params[0].to_string());
                    meta_raw.insert("owner".to_string(), event.params[1].to_string());
                    meta_raw.insert("expires".to_string(), event.params[2].to_string());
                } else if format!("0x{:x}", event.topic0)
                    == "0x3da24c024582931cfaf8267d8ed24d13a82a8068d5bd337d30ec45cea4e506ae"
                {
                    //Source[0].Name renew
                    meta_raw.insert("label".to_string(), event.params[0].to_string());
                    meta_raw.insert("baseCost".to_string(), event.params[1].to_string());
                    meta_raw.insert("expires".to_string(), event.params[2].to_string());
                } else if format!("0x{:x}", event.topic0)
                    == "0x69e37f151eb98a09618ddaa80c8cfaf1ce5996867c489f45b555b412271ebf27"
                {
                    //Source[0].NameRegistered
                    meta_raw.insert("name".to_string(), event.params[0].to_string());
                    meta_raw.insert("label".to_string(), event.params[1].to_string());
                    meta_raw.insert(
                        "owner".to_string(),
                        format!("{}{}", "0x", event.params[2].to_string()),
                    );
                    meta_raw.insert("baseCost".to_string(), event.params[3].to_string());
                    meta_raw.insert("premium".to_string(), event.params[4].to_string());
                    meta_raw.insert("expires".to_string(), event.params[5].to_string());
                } else if format!("0x{:x}", event.topic0)
                    == "0xca6abbe9d7f11422cb6ca7629fbf6fe9efb1c621f71ce8f02b9f2a230097404f"
                {
                    meta_raw.insert("name".to_string(), event.params[0].to_string());
                    meta_raw.insert("label".to_string(), event.params[1].to_string());
                    meta_raw.insert(
                        "owner".to_string(),
                        format!("{}{}", "0x", event.params[2].to_string()),
                    );
                    meta_raw.insert("baseCost".to_string(), event.params[3].to_string());
                    meta_raw.insert("expires".to_string(), event.params[4].to_string());
                } else if format!("0x{:x}", event.topic0)
                    == "0x3da24c024582931cfaf8267d8ed24d13a82a8068d5bd337d30ec45cea4e506ae"
                {
                    meta_raw.insert("name".to_string(), event.params[0].to_string());
                    meta_raw.insert("label".to_string(), event.params[1].to_string());
                    meta_raw.insert("baseCost".to_string(), event.params[2].to_string());
                    meta_raw.insert("expires".to_string(), event.params[3].to_string());
                }
            }
            info!(
                "meta raw is {:?} {:?}",
                meta_raw, transaction_indexed.transaction.txn_hash
            );
            // let meta_modified: Meta = Meta {
            //     id: Some(meta_raw["name"].clone()),
            //     owner: Some(meta_raw["owner"].parse::<H160>().unwrap()),
            //     title: Some(format!("{}.eth", meta_raw["name"])),
            //     image: Some(meta_raw["image"].clone()),
            //     content: None,
            // };

            // let meta_indexed = MetaIndexed {
            //     owner: meta_raw["owner"].parse::<H160>().unwrap(),
            //     id: meta_raw["name"].clone(),
            //     slug: schema.slug.clone(),
            //     raw: meta_raw,
            //     modified: Some(meta_modified),
            //     //TODO: Fix these values
            //     createdAt: "".to_string(),
            //     updatedAt: "".to_string(),
            // };

            // let result: MetaResult = MetaResult {
            //     id: transaction_event.params[0].to_string(),
            //     owner: transaction_event.params[2].to_string(),
            //     slug: schema.slug.clone(),
            //     insert: Some(meta_indexed),
            //     update: None,
            //     source: None,
            // };
            return None;
        }
    }
}
