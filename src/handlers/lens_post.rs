use web3::contract::ens::Ens;

use crate::structs::index::{Meta, MethodParam};
use crate::structs::meta::{self, MetaDataStruct, MetaIndexedStruct};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct LensPostMeta {
    profileId: String,
    contentURI: String,
    collectModule: String,
    collectModuleData: String,
    referenceModule: String,
}

pub fn handler_lens_post(transaction_indexed: &TransactionIndexed) -> Option<MetaDataStruct> {
    if transaction_indexed.method.name == "post" {
        let meta_raw: LensPostMeta = LensPostMeta {
            profileId: transaction_indexed.method.params[0].value.clone(),
            contentURI: transaction_indexed.method.params[1].value.clone(),
            collectModule: transaction_indexed.method.params[2].value.clone(),
            collectModuleData: transaction_indexed.method.params[3].value.clone(),
            referenceModule: transaction_indexed.method.params[4].value.clone(),
        };

        let mut image = String::from("https://i.seadn.io/gae/S67RadRtlIbTNk0UojZM-TEl4pybcblKyg3HxQHl0-JmxYZ2deLX-pK2Z89khCWHGeaXeYfE8Vxqj06YCUcqk0q1KWD9T997lGnGHw?auto=format&dpr=1&w=3840");

        let meta_indexed: MetaIndexedStruct = MetaIndexedStruct {
            id: Some(meta_raw.profileId.clone()),
            owner: Some(format!(meta_raw.profileId.clone())),
            title: Some(format!(meta_raw.contentURI.clone())),
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
