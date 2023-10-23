use log::{debug, error, info, warn};
use std::process::exit;

use crate::{
    handlers::poap_ethereum::handler_poap_ethereum,
    structs::{
        contracts::ContractMetaData, extract::Schema, meta::MetaIndexed,
        transactions::TransactionIndexed,
    },
};

pub async fn utils_meta_indexed(
    schema: &Schema,
    transaction_indexed: TransactionIndexed,
) -> MetaIndexed {
    let meta_indexed: MetaIndexed = match handler_poap_ethereum(&transaction_indexed).await {
        Some(object) => {
            let meta_indexed: MetaIndexed = MetaIndexed {
                owner: object.modified.owner.clone().unwrap(),
                id: object.modified.id.clone().unwrap(),
                slug: schema.slug.clone(),
                data: object.clone(),
                createdAt: String::from(""),
                updatedAt: String::from(""),
                sources: vec![transaction_indexed],
            };
            meta_indexed
        }
        None => {
            error!("handler returned null");
            exit(1)
        }
    };
    info!("\nmeta indexed {:?}\n", meta_indexed);
    meta_indexed
}
