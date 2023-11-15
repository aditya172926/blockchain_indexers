use std::collections::HashMap;

use crate::structs::{extract::Schema, meta::MetaResult, transactions::TransactionIndexed};

use log::{debug, error, info, warn};

use super::handler::handler_txn_transfer_poap;

pub async fn handler_poap(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    match &transaction_indexed.method {
        Some(transaction_indexed_method) => {
            if transaction_indexed_method.name == "transferFrom"
                || transaction_indexed_method.name == "safeTransferFrom"
            {
                return handler_txn_transfer_poap(transaction_indexed, schema).await;
            } else {
                None
            }
        }

        None => {
            let mut meta_raw: HashMap<String, String> = HashMap::new();

            for event in transaction_indexed.events.as_ref().unwrap() {
                if format!("0x{:x}", event.topic0)
                    == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                {
                    //source[1].Transfer (index_topic_1 address from, index_topic_2 address to, index_topic_3 uint256 tokenId)
                    meta_raw.insert("from".to_string(), format!("0x{}", event.params[0]));
                    meta_raw.insert("to".to_string(), format!("0x{}", event.params[1]));
                    meta_raw.insert("tokenId".to_string(), event.params[2].to_string());
                }
            }

            // info!(
            //     "meta raw is {:?} \n txn hash is :  {:?}",
            //     meta_raw, transaction_indexed.transaction.txn_hash
            // );
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
