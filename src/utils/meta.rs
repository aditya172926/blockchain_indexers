use std::process::exit;

use log::{debug, error, info, warn};

use crate::{
    handlers::ens_ethereum::handler_ens,
    structs::{
        extract::Schema,
        meta::{MetaIndexed, MetaResult},
        transactions::TransactionIndexed,
    },
};

pub async fn utils_meta_indexed(
    schema: &Schema,
    transaction_indexed: TransactionIndexed,
) -> Option<MetaResult> {
    let fname = &schema.source[0].handlersMethods;
    print!("the fname: {}", fname);

    let mut meta_result: MetaResult = MetaResult::default();
    match fname.as_ref() {
        "handler_ens" => match handler_ens(&transaction_indexed, schema).await {
            Some(object) => {
                info!("\n\n meta result {:?}\n\n", object);
                Some(object)
            }
            None => {
                warn!("handler returned null");
                None
            }
        },
        // "handler_lens_post" => match handler_lens_post(&transaction_indexed).await {
        //     Some(object) => {
        //         meta_indexed = MetaIndexed {
        //             owner: object.clone().modified.unwrap().owner.unwrap(),
        //             id: object.clone().modified.unwrap().id.unwrap(),
        //             slug: schema.slug.clone(),
        //             data: object.clone(),
        //             createdAt: transaction_indexed.clone().timestamp,
        //             updatedAt: String::from(""),
        //             sources: vec![transaction_indexed],
        //         };
        //         Some(meta_indexed)
        //     }
        //     None => {
        //         warn!("handler returned null");
        //         None
        //     }
        // },
        // "handler_lens_profile" => match handler_lens_profile(&transaction_indexed).await {
        //     Some(object) => {
        //         meta_indexed = MetaIndexed {
        //             owner: object.clone().modified.unwrap().owner.unwrap(),
        //             id: object.clone().modified.unwrap().id.unwrap(),
        //             slug: schema.slug.clone(),
        //             data: object.clone(),
        //             createdAt: transaction_indexed.clone().timestamp,
        //             updatedAt: String::from(""),
        //             sources: vec![transaction_indexed],
        //         };
        //         Some(meta_indexed)
        //     }
        //     None => {
        //         warn!("handler returned null");
        //         None
        //     }
        // },
        // "handler_poap_ethereum" => match handler_poap_ethereum(&transaction_indexed).await {
        //     Some(object) => {
        //         meta_indexed = MetaIndexed {
        //             owner: object.clone().modified.unwrap().owner.unwrap(),
        //             id: object.clone().modified.unwrap().id.unwrap(),
        //             slug: schema.slug.clone(),
        //             data: object.clone(),
        //             createdAt: transaction_indexed.clone().timestamp,
        //             updatedAt: String::from(""),
        //             sources: vec![transaction_indexed],
        //         };
        //         Some(meta_indexed)
        //     }
        //     None => {
        //         warn!("handler returned null");
        //         None
        //     }
        // },
        _ => return Some(meta_result),
    }
    // let meta_indexed: Option<MetaIndexed> =
    // info!("\nmeta indexed {:?}\n", meta_indexed);
    // meta_indexed
}
