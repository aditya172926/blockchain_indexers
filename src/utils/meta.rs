use std::process::exit;

use crate::{
    handlers::ens_ethereum::index,
    handlers::lens_profile_polygon::index::load_lens_profile_event,
    handlers::{poap_ethereum::index::load_poap_event, sound_optimism::index::load_sound_event},
    structs::{
        contracts::ContractIndexed,
        extract::Schema,
        meta::{MetaIndexed, MetaResult},
        transactions::TransactionIndexed,
    },
};
use log::{debug, error, info, warn};

pub async fn utils_meta_indexed(
    schema: &Schema,
    transaction_indexed: TransactionIndexed,
    contracts: &mut Vec<ContractIndexed>,
) -> Option<MetaResult> {
    // The index should come from config
    let fname: &str = &schema.source[0].handlersMethods;
    // print!("the fname: {}", fname);

    let mut meta_result: MetaResult = MetaResult::default();
    // Some(meta_result)
    match fname.as_ref() {
        "handler_ens" => match index::load_ens_event(schema, contracts, &transaction_indexed).await
        {
            Some(object) => {
                info!("\n\n meta result {:?}\n\n", object);
                // Some(object)
                None
            }
            None => {
                warn!("ens handler returned null");
                None
            }
        },
        "handler_lens_profile" => {
            match load_lens_profile_event(schema, contracts, &transaction_indexed).await {
                Some(object) => {
                    info!("\n\n meta result {:?}\n\n", object);
                    // Some(object)
                    None
                }
                None => {
                    warn!("lens profile handler returned null");
                    None
                }
            }
        }
        "handler_poap_ethereum" => {
            match load_poap_event(schema, contracts, &transaction_indexed).await {
                Some(object) => {
                    info!("\n\n meta result {:?}\n\n", object);
                    // Some(object)
                    None
                }
                None => {
                    warn!("lens profile handler returned null");
                    None
                }
            }
        }
        "handler_sound" => {
            match load_sound_event(schema, contracts, &transaction_indexed).await {
                Some(object) => {
                    info!("\n\n meta result {:?}\n\n", object);
                    // Some(object)
                    None
                }
                None => {
                    warn!("lens profile handler returned null");
                    None
                }
            }
        }
        _ => return Some(meta_result),
    }
}
