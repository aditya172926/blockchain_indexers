use ethers::types::H160;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::process::exit;

use crate::handlers::ens_ethereum::handler;
use crate::structs::extract::Schema;
use crate::structs::meta::{self, Meta, MetaIndexed, MetaResult};
use crate::structs::transactions::{TransactionEvent, TransactionIndexed};

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

            println!(" txn indexed : \n{:?}\n\n", transaction_indexed);
            for event in transaction_indexed.events.as_ref().unwrap() {
                if let Some(data) = event.data.clone() {
                    for (key, value) in data.iter() {
                        meta_raw.insert(key.to_string(), value.to_string());
                    }
                }
            }
            info!("meta raw is {:?}", meta_raw);
            let meta_modified: Meta = Meta {
                id: Some(meta_raw["name"].clone()),
                owner: Some(meta_raw["owner"].parse::<H160>().unwrap()),
                title: Some(format!("{}.eth", meta_raw["name"])),
                image: None,
                content: None,
            };

            let meta_indexed = MetaIndexed {
                owner: meta_raw["owner"].parse::<H160>().unwrap(),
                id: meta_raw["name"].clone(),
                slug: schema.slug.clone(),
                raw: meta_raw.clone(),
                modified: Some(meta_modified),
                //TODO: Fix these values
                createdAt: "".to_string(),
                updatedAt: "".to_string(),
            };

            let result: MetaResult = MetaResult {
                id: meta_raw["name"].clone(),
                owner: meta_raw["owner"].clone(),
                slug: schema.slug.clone(),
                insert: Some(meta_indexed),
                update: None,
                source: None,
            };
            return Some(result);
        }
    }
}
