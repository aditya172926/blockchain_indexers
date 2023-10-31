use ethers::types::H160;
use std::collections::HashMap;
use std::process::exit;
use log::{debug, error, info, warn};

use crate::structs::extract::Schema;
use crate::structs::meta::{Meta, MetaIndexed, MetaResult};
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

#[derive(Debug)]
struct EnsEventMeta{
    name:String,
    label:String,
    owner:H160,
    baseCost:String,
    premium:String,
    expires:String
}

pub async fn method_handler(
    name: String,
    content: String,
    owner: H160,
    duration: String,
    secret: String,
    resolver: String,
    data: String,
    reverseRecord: String,
    ownerControlledFuses: String,
    slug: String,
) -> Option<MetaIndexed> {
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
    let mut raw_data: HashMap<String, String> = HashMap::from([
        (String::from("owner"), format!("0x{:x}", meta_raw.owner)),
        (String::from("name"), meta_raw.name),
        (String::from("duration"), meta_raw.duration),
    ]);

    let image = String::from(
        "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg",
    );
    let meta_modified: Meta = Meta {
        id: Some(name.clone()),
        owner: Some(meta_raw.owner),
        title: Some(format!("{}.eth", raw_data.get("name").unwrap())),
        image: Some(image),
        content: None,
    };

    let meta_indexed = MetaIndexed {
        owner: owner,
        id: name,
        slug,
        raw: raw_data,
        modified: Some(meta_modified),
        createdAt: "".to_string(),
        updatedAt: "".to_string(),
    };

    // println!("\n\n\nMeta indexed {:?} \n\n\n", meta);

    return Some(meta_indexed);
}

pub async fn event_handler(
    name: String,
    label:String,
    owner: H160,
    baseCost:String,
    premium:String,
    resolver: String,
    expires:String,
    slug: String,
) -> Option<MetaIndexed> {
    let meta_raw: EnsEventMeta = EnsEventMeta {
        name: name.clone(),
        label,
        owner,
        baseCost,
        premium,
        expires,
    };
    let mut raw_data: HashMap<String, String> = HashMap::from([
        (String::from("owner"), format!("0x{:x}", meta_raw.owner)),
        (String::from("name"), meta_raw.name),
        (String::from("expiration"), meta_raw.expires),
    ]);

    let image = String::from(
        "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg",
    );
    let meta_modified: Meta = Meta {
        id: Some(name.clone()),
        owner: Some(meta_raw.owner),
        title: Some(format!("{}.eth", raw_data.get("name").unwrap())),
        image: Some(image),
        content: None,
    };

    let meta_indexed = MetaIndexed {
        owner: owner,
        id: name,
        slug,
        raw: raw_data,
        modified: Some(meta_modified),
        createdAt: "".to_string(),
        updatedAt: "".to_string(),
    };

    return Some(meta_indexed);
}



pub async fn handler_ens(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    match &transaction_indexed.method{
        Some(transaction_indexed_method)=>{
            if transaction_indexed_method.name == "register"
                || transaction_indexed_method.name == "registerOnly"
            {
                let meta_data = method_handler(
                    transaction_indexed_method.params[0].to_string(),
                    format!(
                        "This ens handle is owned by {} ",
                        transaction_indexed_method.params[1]
                    ),
                    transaction_indexed_method.params[1]
                        .clone()
                        .into_address()
                        .unwrap(),
                    transaction_indexed_method.params[2].to_string(),
                    transaction_indexed_method.params[3].to_string(),
                    transaction_indexed_method.params[4].to_string(),
                    transaction_indexed_method.params[5].to_string(),
                    transaction_indexed_method.params[6].to_string(),
                    transaction_indexed_method.params[7].to_string(),
                    schema.slug.clone(),
                )
                .await;
        
                let result: MetaResult = MetaResult {
                    id: transaction_indexed_method.params[0].to_string(),
                    owner: transaction_indexed_method.params[1].to_string(),
                    slug: schema.slug.clone(),
                    insert: meta_data,
                    update: None,
                    source: transaction_indexed.clone(),
                };
                return Some(result);
            } else if transaction_indexed_method.name == "renew" {
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
                    source: transaction_indexed.clone(),
                };
                return Some(result);
            } else if transaction_indexed_method.name == "reclaim" {
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
                    source: transaction_indexed.clone(),
                };
                return Some(result);
            } else {
                return None;
            }
            },

        None => {
            let transaction_event=transaction_indexed.event.as_ref().unwrap();
            println!("{}",transaction_event.params[0].to_string());
            if transaction_event.name.to_string()=="NameRegistered"{
                    //   println!("event caught here: {:?}\n",transaction_event);
                      let meta_data = event_handler(
                        transaction_event.params[0].to_string(),
                        format!(
                            "This ens handle is owned by {} ",
                            transaction_event.params[1]
                        ),
                        transaction_event.params[2]
                            .clone()
                            .into_address()
                            .unwrap(),
                            transaction_event.params[2].to_string(),
                            transaction_event.params[3].to_string(),
                            transaction_event.params[4].to_string(),
                            transaction_event.params[5].to_string(),
                        schema.slug.clone(),
                    )
                    .await;
                let result: MetaResult = MetaResult {
                    id: transaction_event.params[0].to_string(),
                    owner: transaction_event.params[2].to_string(),
                    slug: schema.slug.clone(),
                    insert: meta_data,
                    update: None,
                    source: transaction_indexed.clone(),
                };
                return None;
                }else if transaction_event.name.to_string()=="NameRenewed"{
                    let mut update_obj = HashMap::new();
                    update_obj.insert(
                        String::from("document.raw.duration"),
                        transaction_event.params[3].to_string(),
                    );
                    let result = MetaResult {
                        id: transaction_event.params[0].to_string(),
                        owner: String::from(""),
                        slug: schema.slug.clone(),
                        insert: None,
                        update: Some(update_obj),
                        source: transaction_indexed.clone(),
                    };
                    return Some(result);
                }else{
                    return None;
                }
            }
   
        
    
    }
}

  
       

