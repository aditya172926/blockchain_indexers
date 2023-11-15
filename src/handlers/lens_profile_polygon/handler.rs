use std::{collections::HashMap, sync::Arc};

use ethers::{types::H160, providers::{Provider, Http}};
use log::info;
use serde_json::Value;

use crate::{structs::{
    extract::{Schema, Config},
    meta::{self, Meta, MetaIndexed, MetaResult},
    transactions::{TransactionEvent, TransactionIndexed, TransactionMethod}, networks::NetworkStruct,
}, utils::{self, reader}};

#[derive(Debug)]
struct LensMeta {
    to: H160,
    handle: String,
    imageURI: String,
    followModule: String,
    followModuleInitData: String,
    //missing data: followNFTURI
}

pub async fn handler(
    to: H160,
    handle: String,
    imageURI: String,
    followModule: String,
    followModuleInitData: String,
    slug: String,
) -> Option<MetaIndexed> {
    let meta_raw: LensMeta = LensMeta {
        to,
        handle: handle.clone(),
        imageURI,
        followModule,
        followModuleInitData,
    };
    let raw_data = HashMap::from([
        (String::from("to"), format!("0x{:x}", meta_raw.to)),
        ((String::from("handle"), meta_raw.handle.to_owned())),
        (String::from("imageURI"), meta_raw.imageURI.to_owned()),
        (String::from("followModule"), meta_raw.followModule),
        (
            String::from("followModuleInitData"),
            meta_raw.followModuleInitData,
        ),
    ]);

    let mut image;
    if meta_raw.imageURI != "" {
        image = format!("ipfs://{}", meta_raw.imageURI)
    } else {
        image = String::from("https://i.seadn.io/gae/S67RadRtlIbTNk0UojZM-TEl4pybcblKyg3HxQHl0-JmxYZ2deLX-pK2Z89khCWHGeaXeYfE8Vxqj06YCUcqk0q1KWD9T997lGnGHw?auto=format&dpr=1&w=3840")
    }
    let meta_modified: Meta = Meta {
        id: Some(meta_raw.handle.clone()),
        owner: Some(meta_raw.to),
        title: Some(format!("{}.lens", meta_raw.handle.clone())),
        image: Some(image),
        content: None,
    };
    let meta_indexed: MetaIndexed = MetaIndexed {
        owner: to,
        id: handle,
        slug,
        raw: raw_data,
        modified: Some(meta_modified),
        createdAt: "".to_string(),
        updatedAt: "".to_string(),
    };

    // println!("\n\n\nMeta indexed {:?} \n\n\n", meta);

    return Some(meta_indexed);
}

pub async fn handler_txn_create_profile_lens(transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult>{
    let transaction_indexed_method=transaction_indexed.method.clone().unwrap();
        let params_list = transaction_indexed_method.params[0]
        .clone()
        .into_tuple()
        .unwrap();
    info!("params_list -> {:?}", params_list);
    let meta_data = handler(
        params_list[0].clone().into_address().unwrap(),
        params_list[1].to_string(),
        params_list[2].to_string(),
        params_list[3].to_string(),
        params_list[4].to_string(),
        schema.slug.clone(),
    )
    .await;
    let result: MetaResult = MetaResult {
        id: params_list[0].to_string(),
        owner: params_list[1].to_string(),
        slug: schema.slug.clone(),
        insert: meta_data,
        update: None,
        source: Some(transaction_indexed.clone()),
    };
    return Some(result);
}

pub async fn handler_txn_set_profile_lens(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {

    let transaction_indexed_method=transaction_indexed.method.clone().unwrap();
        let params_list = transaction_indexed_method.params[0]
        .clone()
        .into_tuple()
        .unwrap();
    let update_obj: HashMap<String, String> = HashMap::from([
        (
            String::from("document.raw.imageURI"),
            params_list[1].to_string(),
        ),
        (
            String::from("document.modified.image"),
            params_list[1].to_string(),
        ),
    ]);
    let result = MetaResult {
        id: params_list[0].to_string(),
        owner: transaction_indexed.transaction.from.unwrap().to_string(),
        slug: schema.slug.clone(),
        insert: None,
        update: Some(update_obj),
        source: Some(transaction_indexed.clone()),
    };
return Some(result);
}


pub async fn handler_event_create_profile(
    metadata:serde_json::Value,
    transaction_indexed: &TransactionIndexed,
    event:&TransactionEvent,
    schema: &Schema,
) -> Option<MetaResult> {
     //Transfer (index_topic_1 address from, index_topic_2 address to, index_topic_3 uint256 tokenId)
    let mut meta_raw=HashMap::new();
     meta_raw.insert("owner".to_string(), event.params[1].to_string());
     meta_raw.insert("tokenId".to_string(), event.params[2].clone().into_uint().unwrap().to_string());
     meta_raw.insert("imageUri".to_string(), metadata["image"].to_string());
     meta_raw.insert("description".to_string(), metadata["description"].to_string());
 
        let meta_modified:Meta = Meta {
            id: Some(metadata["name"].to_string()),
            owner: Some(meta_raw["owner"].clone().parse::<H160>().unwrap()),
            title: Some(format!("{}.lens", meta_raw["owner"])),
            image: Some(meta_raw["imageUri"].clone()),
            content: None,
        };

        let meta_indexed = MetaIndexed {
            owner: meta_raw["owner"].clone().parse::<H160>().unwrap(),
            id: metadata["name"].to_string(),
            slug: schema.slug.clone(),
            raw: meta_raw.clone(),
            modified: Some(meta_modified),
            //TODO: Fix these values
            createdAt: "".to_string(),
            updatedAt: "".to_string(),
        };

        let result = MetaResult {
            id: metadata["name"].to_string(),
            owner: meta_raw["owner"].clone(),
            slug: schema.slug.clone(),
            insert: Some(meta_indexed),
            update: None,
            source: Some(transaction_indexed.clone()),
        };
        println!("FOUND RESULT----------{:?}\n",result);
    return Some(result);
}

