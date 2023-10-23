use log::{debug, error, info, warn};
use std::process::exit;

use crate::{
    handlers::poap_ethereum::handler_poap_ethereum,
    structs::{
        contracts::ContractMetaData, extract::Config, meta::MetaIndexed,
        transactions::TransactionIndexed,
    },
};

pub async fn utils_meta_indexed(
    config: &Config,
    transaction_indexed: TransactionIndexed,
) -> Option<MetaIndexed> {
    let meta_indexed: Option<MetaIndexed> = match handler_poap_ethereum(&transaction_indexed).await {
        Some(object) => {
            let meta_indexed: MetaIndexed = MetaIndexed {
                owner: object.modified.owner.clone().unwrap(),
                id: object.modified.id.clone().unwrap(),
                slug: config.slug.clone(),
                data: object.clone(),
                createdAt: String::from(""),
                updatedAt: String::from(""),
                sources: vec![transaction_indexed],
            };
            Some(meta_indexed)
        }
        None => {
            warn!("handler returned null");
            None
        }
    };
    info!("\nmeta indexed {:?}\n", meta_indexed);
    meta_indexed
}
