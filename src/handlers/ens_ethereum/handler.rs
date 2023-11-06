use std::{collections::HashMap, sync::Arc};

use ethers::{
    providers::reverse_address,
    types::{H160, U256},
};

use crate::{
    structs::{
        extract::Schema,
        meta::{self, Meta, MetaIndexed, MetaResult},
        transactions::{TransactionEvent, TransactionIndexed, TransactionMethod},
    },
    utils::index::utils_url_data,
};
use ethers::{
    abi::{token, Hash},
    contract::ContractInstance,
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, Bytes, Filter, ValueOrArray, H256},
};
use log::{error, info};

pub async fn handler_txn_register_ens(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    let transaction_indexed_method = transaction_indexed.method.clone().unwrap();
    let mut meta_raw: HashMap<String, String> = HashMap::from([
        (
            String::from("name"),
            transaction_indexed_method.params[0].to_string(),
        ),
        (
            String::from("owner"),
            format!("{}{}", "0x", transaction_indexed_method.id.to_string()),
        ),
        (
            String::from("duration"),
            transaction_indexed_method.params[2].to_string(),
        ),
        (
            String::from("secret"),
            transaction_indexed_method.params[2].to_string(),
        ),
        (
            String::from("resolver"),
            transaction_indexed_method.params[2].to_string(),
        ),
        (
            String::from("data"),
            transaction_indexed_method.params[2].to_string(),
        ),
        (
            String::from("reverseRecord"),
            transaction_indexed_method.params[2].to_string(),
        ),
        (
            String::from("ownerControlledFuses"),
            transaction_indexed_method.params[2].to_string(),
        ),
        (
            String::from("image"),
            "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg"
                .to_owned(),
        ),
    ]);

    let meta_modified: Meta = Meta {
        id: Some(meta_raw["name"].clone()),
        owner: Some(meta_raw["owner"].parse::<H160>().unwrap()),
        title: Some(format!("{}.eth", meta_raw["name"])),
        image: Some(meta_raw["image"].clone()),
        content: None,
    };

    let meta_indexed = MetaIndexed {
        owner: meta_raw["owner"].parse::<H160>().unwrap(),
        id: meta_raw["name"].clone(),
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

pub async fn handler_txn_renew_ens(
    transaction_indexed: &TransactionIndexed,
    transaction_indexed_method: &TransactionMethod,
    schema: &Schema,
) -> Option<MetaResult> {
    let mut update_obj = HashMap::new();
    update_obj.insert(
        String::from("document.raw.duration"),
        transaction_indexed_method.params[1].to_string(),
    );
    let result = MetaResult {
        id: transaction_indexed_method.params[0].to_string(),
        owner: transaction_indexed.transaction.from.unwrap().to_string(),
        slug: schema.slug.clone(),
        insert: None,
        update: Some(update_obj),
        source: Some(transaction_indexed.clone()),
    };
    return Some(result);
}

pub async fn handler_txn_reclaim_ens(
    transaction_indexed: &TransactionIndexed,
    transaction_indexed_method: &TransactionMethod,
    schema: &Schema,
) -> Option<MetaResult> {
    let update_obj: HashMap<String, String> = HashMap::from([
        (
            String::from("document.owner"),
            transaction_indexed_method.params[1].to_string(),
        ),
        (
            String::from("document.raw.owner"),
            transaction_indexed_method.params[1].to_string(),
        ),
        (
            String::from("document.modified.owner"),
            transaction_indexed_method.params[1].to_string(),
        ),
    ]);

    let result = MetaResult {
        id: transaction_indexed_method.params[0].to_string(),
        owner: transaction_indexed_method.params[1].to_string(),
        slug: schema.slug.clone(),
        insert: None,
        update: Some(update_obj),
        source: Some(transaction_indexed.clone()),
    };
    return Some(result);
}

/**
    NameRegistered(indexed uint256,indexed address,uint256)
    params:
        hash: uint256
        owner: address
        expires: uint256
**/
pub async fn handler_event_register_ens_by_controller(
    event: &TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();
    meta_raw.insert("name".to_string(), event.params[0].to_string());
    meta_raw.insert("label".to_string(), event.params[1].to_string());
    meta_raw.insert(
        "owner".to_string(),
        format!("{}{}", "0x", event.params[2].to_string()),
    );
    meta_raw.insert("baseCost".to_string(), event.params[3].to_string());
    meta_raw.insert("premium".to_string(), event.params[4].to_string());
    meta_raw.insert("expires".to_string(), event.params[5].to_string());
    return meta_raw;
}

pub async fn handler_event_register_ens_by_controller_old(
    event: &TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();
    meta_raw.insert("name".to_string(), event.params[0].to_string());
    meta_raw.insert("label".to_string(), event.params[1].to_string());
    meta_raw.insert(
        "owner".to_string(),
        format!("{}{}", "0x", event.params[2].to_string()),
    );
    meta_raw.insert("baseCost".to_string(), event.params[3].to_string());
    meta_raw.insert("expires".to_string(), event.params[4].to_string());
    return meta_raw;
}

pub async fn handler_event_register_ens_by_base(
    event: &TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();
    meta_raw.insert("tokenId".to_string(), event.params[0].to_string());
    meta_raw.insert("owner".to_string(), event.params[1].to_string());
    meta_raw.insert("expires".to_string(), event.params[2].to_string());
    return meta_raw;
}

pub async fn handler_event_renew_ens_by_controller(
    event: &TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();
    meta_raw.insert("name".to_string(), event.params[0].to_string());
    meta_raw.insert("label".to_string(), event.params[1].to_string());
    meta_raw.insert("baseCost".to_string(), event.params[2].to_string());
    meta_raw.insert("expires".to_string(), event.params[3].to_string());
    return meta_raw;
}

pub async fn handler_event_renew_ens_by_base(
    contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    event: &TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();
    meta_raw.insert("tokenId".to_string(), event.params[0].to_string());
    meta_raw.insert("expires".to_string(), event.params[1].to_string());
    let tokeId_decimal = U256::from_str_radix(&event.params[0].to_string(), 16).unwrap();
    println!("token id {}", tokeId_decimal);
    let owner = contract_instance.method::<_, String>("ownerOf", tokeId_decimal);
    println!("owner_address_promise : {:?} ", owner);
    let owner_address = match owner {
        Ok(method) => {
            let owner_address_promise = method.call().await;
            match owner_address_promise {
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
    meta_raw.insert("owner".to_string(), owner_address);

    return meta_raw;
}

pub async fn handler_event_renew_ens_by_controller_old(
    event: TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();
    meta_raw.insert("name".to_string(), event.params[0].to_string());
    meta_raw.insert("label".to_string(), event.params[1].to_string());
    meta_raw.insert("baseCost".to_string(), event.params[2].to_string());
    meta_raw.insert("expires".to_string(), event.params[3].to_string());
    return meta_raw;
}

pub async fn handler_event_transfer_ens_by_base(
    contract_instance: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    event: TransactionEvent,
) -> HashMap<String, String> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();

    meta_raw.insert("from".to_string(), event.params[0].to_string());
    meta_raw.insert("to".to_string(), event.params[1].to_string());
    meta_raw.insert("tokenId".to_string(), event.params[2].to_string());

    let name = reverse_address(event.params[1].to_string().parse::<H160>().unwrap());

    meta_raw.insert("name".to_string(), name);
    meta_raw.insert("owner".to_string(), event.params[1].to_string());

    return meta_raw;
}
