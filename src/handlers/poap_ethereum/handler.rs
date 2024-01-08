use std::collections::HashMap;
use std::sync::Arc;

use crate::structs::extract::Schema;
use crate::structs::meta::{self, Meta, MetaIndexed, MetaResult};
use crate::structs::transactions::{TransactionEvent, TransactionIndexed};
use crate::utils::index::utils_url_data;

use ethers::types::{H160, U128, U256};
use ethers::{
    abi::{token, Hash},
    contract::ContractInstance,
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, Bytes, Filter, ValueOrArray, H256},
};
use log::{error, info};
use serde_json::json;

pub async fn handler_txn_transfer_poap(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    let transaction_indexed_method = transaction_indexed.method.clone().unwrap();
    let mut meta_raw: HashMap<String, String> = HashMap::from([
        (
            String::from("from"),
            format!("{}", transaction_indexed_method.params[0]),
        ),
        (
            String::from("to"),
            format!("{}", transaction_indexed_method.params[1]),
        ),
        (
            String::from("tokenId"),
            transaction_indexed_method.params[2].to_string(),
        ),
    ]);

    let mut image = String::new();
    let meta_modified: Meta = Meta {
        id: Some(meta_raw["tokenId"].clone()),
        owner: Some(H160::from_slice(&meta_raw["to"].clone().into_bytes())),
        title: Some(meta_raw["tokenId"].clone()),
        image: Some(image),
        content: None,
    };

    let meta_indexed = MetaIndexed {
        owner: meta_modified.owner.clone().unwrap(),
        id: meta_modified.id.clone().unwrap(),
        slug: schema.slug.clone(),
        raw: meta_raw,
        modified: Some(meta_modified),
        createdAt: "".to_string(),
        updatedAt: "".to_string(),
    };

    let result: MetaResult = MetaResult {
        id: transaction_indexed_method.params[0].to_string(),
        owner: transaction_indexed_method.params[1].to_string(),
        slug: schema.slug.clone(),
        insert: Some(meta_indexed),
        update: None,
        source: Some(transaction_indexed.clone()),
    };
    return Some(result);
}

pub async fn handler_transfer_poap(
    meta_raw: &mut HashMap<String, String>,
    contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    transaction_event: TransactionEvent,
) {
    let tokenUrl =
        contract_instance.method::<_, String>("tokenURI", transaction_event.params[2].clone());
    let token_url = match tokenUrl {
        Ok(method) => {
            let token_url_promise = method.call().await;
            match token_url_promise {
                Ok(result) => result,
                Err(e) => {
                    error!("Error in contract call -> {:?}\n\n", e);
                    String::new()
                }
            }
        }
        Err(e) => {
            error!(
                "Error in get_token_url method from the smart contract {:?}\n\n",
                e
            );
            String::new()
        }
    };
    let token_url_data = utils_url_data(&token_url).await;

    meta_raw.insert(
        "from".to_string(),
        format!("0x{}", transaction_event.params[0]),
    );
    meta_raw.insert(
        "to".to_string(),
        format!("0x{}", transaction_event.params[1]),
    );
    meta_raw.insert(
        "tokenId".to_string(),
        transaction_event.params[2].to_string(),
    );
    meta_raw.insert("tokenUrl".to_string(), token_url.clone());
    match token_url_data {
        Some(object) => {
            meta_raw.insert("description".to_string(), object["description"].to_string());
            meta_raw.insert(
                "external_url".to_string(),
                object["external_url"].to_string(),
            );
            meta_raw.insert("home_url".to_string(), object["home_url"].to_string());
            meta_raw.insert("image_url".to_string(), object["image_url"].to_string());
            meta_raw.insert("name".to_string(), object["name"].to_string());
        }
        None => {}
    }
}
