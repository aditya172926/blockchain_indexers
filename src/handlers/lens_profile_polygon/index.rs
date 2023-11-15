use std::{collections::HashMap, process::exit};

use ethers::types::H160;
use log::info;

use crate::{
    handlers::{
        ens_ethereum::handler::{
            handler_event_register_ens_by_base, handler_event_register_ens_by_controller,
            handler_event_register_ens_by_controller_old, handler_event_renew_ens_by_base,
            handler_event_renew_ens_by_controller, handler_event_transfer_ens_by_base,
        },
        poap_ethereum::handler::handler_transfer_poap,
    },
    structs::{
        contracts::ContractIndexed,
        extract::Schema,
        meta::MetaIndexed,
        transactions::{TransactionEvent, TransactionIndexed},
    },
};

use super::handler::handler_event_create_profile;

pub async fn load_lens_profile_event(
    schema: &Schema,
    contracts: &mut Vec<ContractIndexed>,
    transaction_indexed: &TransactionIndexed,
) -> Option<HashMap<String, String>> {
    let mut meta_raw: HashMap<String, String> = HashMap::new();

    for event in transaction_indexed.events.as_ref().unwrap() {
        if schema.slug == "lens_profile_polygon"
            && format!("0x{:x}", event.topic0)
                == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        {
            handler_event_create_profile(
                &mut meta_raw,
                contracts[1].instance.clone(),
                &event.clone(),
            )
            .await;
        }
    }
    info!("meta raw is : {:?} ", meta_raw);

    let meta_indexed = MetaIndexed {
        owner: meta_raw["owner"].parse::<H160>().unwrap(),
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
