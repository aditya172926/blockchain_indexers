use std::process::exit;

use crate::{
    handlers::poap_ethereum::handler_poap_ethereum,
    structs::{
        contracts::ContractMetaData,
        meta::{MetaIndexed, MetaSubStruct},
        transactions::TransactionIndexed,
    },
};

pub async fn utils_meta_indexed(
    transaction_indexed: TransactionIndexed,
    contract_metadata: ContractMetaData,
) -> MetaIndexed {
    let meta_indexed = match handler_poap_ethereum(&transaction_indexed).await {
        Some(object) => {
            let meta_sub_struct: MetaSubStruct = MetaSubStruct {
                data: object.clone(),
            };
            let meta_indexed: MetaIndexed = MetaIndexed {
                metaOwner: object.modified.owner.unwrap(),
                metaId: object.modified.id.unwrap(),
                slug: contract_metadata.contract_slug,
                meta: meta_sub_struct,
                createdAt: String::from(""),
                updatedAt: String::from(""),
                sources: vec![transaction_indexed],
            };
            meta_indexed
        }
        None => {
            println!("handler returned null");
            exit(1)
        }
    };
    return meta_indexed;
    println!("\n\n\n\n\n meta block {:?} \n\n\n\n\n", meta_indexed);
}
