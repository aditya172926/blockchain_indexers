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
