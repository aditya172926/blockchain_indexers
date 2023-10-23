use std::process::exit;
use log::{debug, error, info, warn};

use crate::{
    handlers::{poap_ethereum::handler_poap_ethereum, lens_post::handler_lens_post},
    structs::{
        contracts::ContractMetaData,
        meta::{MetaIndexed, MetaSubStruct},
        transactions::TransactionIndexed, extract::Config,
    },
};

pub async fn utils_meta_indexed(
    config: &Config,
    transaction_indexed: TransactionIndexed,
) -> Option<MetaIndexed> {
    let meta_indexed: Option<MetaIndexed> = match handler_lens_post(&transaction_indexed).await {
        Some(object) => {
            let meta_sub_struct: MetaSubStruct = MetaSubStruct {
                data: object.clone(),
            };
            let meta_indexed: MetaIndexed = MetaIndexed {
                metaOwner: object.modified.owner.unwrap(),
                metaId: object.modified.id.unwrap(),
                slug: config.slug.clone(),
                meta: meta_sub_struct,
                createdAt: String::from(""),
                updatedAt: String::from(""),
                sources: vec![transaction_indexed],
            };
            info!("\nmeta indexed {:?}\n", meta_indexed);
            Some(meta_indexed)
        },
        None => {
            error!("handler returned null");
            None
        }
    };
    meta_indexed
}
