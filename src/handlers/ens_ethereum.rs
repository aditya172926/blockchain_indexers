use ethers::types::H160;
use std::collections::HashMap;
use web3::contract::ens::Ens;

use crate::structs::extract::Owner;
use crate::structs::index::MethodParam;
use crate::structs::meta::{self, Meta, MetaData, MetaResult};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct EnsMeta {
    name: String,
    content: String,
    owner: H160,
    duration: String,
    secret: String,
    resolver: String,
    data: String,
    reverseRecord: String,
    ownerControlledFuses: String,
}
pub async fn handler(
    name: String,
    content: String,
    owner: H160,
    duration: String,
    secret: String,
    resolver: String,
    data: String,
    reverseRecord: String,
    ownerControlledFuses: String,
) -> Option<MetaData> {
    let meta_raw: EnsMeta = EnsMeta {
        name: name.clone(),
        content,
        owner,
        duration,
        secret,
        resolver,
        data,
        reverseRecord,
        ownerControlledFuses,
    };
    let mut raw_data = HashMap::from([
        (String::from("owner"), meta_raw.owner.clone().to_string()),
        (String::from("name"), meta_raw.name),
        (String::from("duration"), meta_raw.duration),
    ]);

    let mut image = String::from(
        "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg",
    );
    let meta_modified: Meta = Meta {
        id: Some(name),
        owner: Some(meta_raw.owner),
        title: Some(format!("{}.eth", raw_data.get("owner").unwrap())),
        image: Some(image),
        content: None,
    };

    meta_indexed = MetaIndexed {
        owner: owner,
        id: name,
        slug: schema.slug.clone(),
        raw: raw_data,
        modified: Some(meta_modified)
        createdAt: transaction_indexed.clone().timestamp,
        updatedAt: String::from(""),
    };

    // println!("\n\n\nMeta indexed {:?} \n\n\n", meta);
    
    return Some(meta_data);
}

pub async fn handler_ens(transaction_indexed: &TransactionIndexed) -> Option<MetaResult> {
    if transaction_indexed.method.name == "register" {
        let meta_data = handler(
            transaction_indexed.method.params[0].to_string(),
            format!(
                "This ens handle is owned by {} ",
                transaction_indexed.method.params[1]
            ),
            transaction_indexed.method.params[1]
                .clone()
                .into_address()
                .unwrap(),
            transaction_indexed.method.params[2].to_string(),
            transaction_indexed.method.params[3].to_string(),
            transaction_indexed.method.params[4].to_string(),
            transaction_indexed.method.params[5].to_string(),
            transaction_indexed.method.params[6].to_string(),
            transaction_indexed.method.params[7].to_string(),
        )
        .await;


        let result = MetaResult{
            id: transaction_indexed.method.params[0].to_string(),
            owner: transaction_indexed.method.params[1].to_string(),
            insert: meta_data,
            update: None,
            source: transaction_indexed,
        };
        return Some(result);

        
    } else if transaction_indexed.method.name == "renew" {
        let result = MetaResult{
            id: transaction_indexed.method.params[0].to_string(),
            owner: transaction_indexed.transaction.from.to_string(),
            insert: None,
            update: Some({"document.raw.duration":  transaction_indexed.method.params[1].to_string()}),
            source: transaction_indexed,
        };
        return Some(result);
    } else {
        return None;
    }
}
