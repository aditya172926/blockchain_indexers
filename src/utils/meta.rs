use std::process::exit;

use log::{debug, error, info, warn};
use crate::{
    handlers::{ens_ethereum::index},
    structs::{
        extract::Schema,
        meta::{MetaIndexed, MetaResult},
        transactions::TransactionIndexed, contracts::ContractIndexed,
    },
};
use crate::handlers::lens_profile_polygon::index::handler_lens_profile;

pub async fn utils_meta_indexed(
    schema: &Schema,
    transaction_indexed: TransactionIndexed,
    contracts: &mut Vec<ContractIndexed>,
) -> Option<MetaResult> {
    // The index should come from config
    let fname = &schema.source[0].handlersMethods;
    // print!("the fname: {}", fname);

    let mut meta_result: MetaResult = MetaResult::default();
    match fname.as_ref() {
        "handler_ens" => match index::handler_ens(&transaction_indexed, schema).await {
            Some(object) => {
                info!("\n\n meta result {:?}\n\n", object);
                Some(object)
            }
            None => {
                warn!("ens handler returned null");
                None
            }
        },
        "handler_lens_profile" => match handler_lens_profile(&transaction_indexed, schema,contracts).await {
            Some(object) => {
                info!("\n\n meta result {:?}\n\n", object);
                Some(object)
            }
            None => {
                warn!("lens profile handler returned null");
                None
            }
        },
        // "handler_poap_ethereum" => {
        //     match handler_poap_ethereum(&transaction_indexed, schema).await {
        //         Some(object) => {
        //             info!("\n\n meta result {:?}\n\n", object);
        //             Some(object)
        //         }
        //         None => {
        //             warn!("lens profile handler returned null");
        //             None
        //         }
        //     }
        // }
        _ => return Some(meta_result),
    }
}
