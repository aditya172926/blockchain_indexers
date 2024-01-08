use std::{collections::HashMap, process::exit};

use ethers::types::H160;
use log::info;

use crate::structs::{
    contracts::ContractIndexed,
    extract::Schema,
    meta::{self, MetaIndexed},
    transactions::{TransactionEvent, TransactionIndexed},
};

use super::handler::handler_event_create_sound;

pub async fn load_sound_event(
    schema: &Schema,
    contracts: &mut Vec<ContractIndexed>,
    transaction_indexed: &TransactionIndexed,
) -> Option<HashMap<String, String>> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();

    for event in transaction_indexed.events.as_ref().unwrap() {
        if schema.slug == "sound_optimism"
            && format!("0x{:x}", event.topic0)
                == "0x81fb8daf8a05fc760e25f1447b0ca819bcf138a168ec6c1aaa0bd62b170bf32a"
        {
            handler_event_create_sound(&mut meta_raw, &event).await;
        }
    }

    info!("meta raw is : {:?}", meta_raw);

    // let meta_indexed = MetaIndexed {
    //     owner: meta_raw["to"].parse::<H160>().unwrap(),
    //     id: meta_raw["tokenId"].clone(),
    //     slug: schema.slug.clone(),
    //     raw: meta_raw,
    //     modified: None,
    //     createdAt: "".to_string(),
    //     updatedAt: "".to_string(),
    // };

    // info!("meta indexed is : {:?} ", meta_indexed);
    None
}
