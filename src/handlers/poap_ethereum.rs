use web3::contract::ens::Ens;

use crate::structs::index::{Meta, MethodParam};
use crate::structs::meta::{self, MetaDataStruct, MetaIndexedStruct};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct PoapMeta {
    from: String,
    to: String,
    tokenId: String,
}

pub fn handler_poap_ethereum(transaction_indexed: &TransactionIndexed) -> Option<MetaDataStruct> {
    if transaction_indexed.method.name == "transferFrom" {
        let meta_raw: PoapMeta = PoapMeta {
            from: transaction_indexed.method.params[0].value.clone(),
            to: transaction_indexed.method.params[1].value.clone(),
            tokenId: transaction_indexed.method.params[2].value.clone(),
        };
        let mut image = String::from("");

        let meta_indexed: MetaIndexedStruct = MetaIndexedStruct {
            id: Some(meta_raw.tokenId.clone()),
            owner: Some(meta_raw.to.clone()),
            title: Some(meta_raw.tokenId.clone()),
            image: Some(image),
        };
        println!("\n\n\nMeta indexed {:?} \n\n\n", meta_indexed);
        let meta_data: MetaDataStruct = MetaDataStruct {
            modified: meta_indexed,
        };
        return Some(meta_data);
    } else {
        return None;
    }
}
