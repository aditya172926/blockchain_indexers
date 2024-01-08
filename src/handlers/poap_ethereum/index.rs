use std::{collections::HashMap, process::exit};

use ethers::types::H160;
use log::info;

use crate::{
    handlers::poap_ethereum::handler::handler_transfer_poap,
    structs::{
        contracts::ContractIndexed,
        extract::Schema,
        meta::{self, MetaIndexed},
        transactions::{TransactionEvent, TransactionIndexed},
    },
};

pub async fn load_poap_event(
    schema: &Schema,
    contracts: &mut Vec<ContractIndexed>,
    transaction_indexed: &TransactionIndexed,
) -> Option<HashMap<String, String>> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();

    for event in transaction_indexed.events.as_ref().unwrap() {
        if schema.slug == "poap_ethereum"
            && format!("0x{:x}", event.topic0)
                == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        {
            handler_transfer_poap(&mut meta_raw, contracts[0].instance.clone(), event.clone())
                .await;
        }
    }

    let meta_indexed = MetaIndexed {
        owner: meta_raw["to"].parse::<H160>().unwrap(),
        id: meta_raw["tokenId"].clone(),
        slug: schema.slug.clone(),
        raw: meta_raw,
        modified: None,
        createdAt: "".to_string(),
        updatedAt: "".to_string(),
    };

    info!("meta indexed is : {:?} ", meta_indexed);
    None
}
