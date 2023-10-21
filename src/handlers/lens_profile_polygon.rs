use web3::contract::ens::Ens;

use crate::structs::index::{Meta, MethodParam};
use crate::structs::meta::{self, MetaDataStruct, MetaIndexedStruct};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct LensMeta {
    to: String,
    handle: String,
    imageURI: String,
    followModule: String,
    followModuleInitData: String,
}

pub fn handler_lens_profile(transaction_indexed: &TransactionIndexed) -> Option<MetaDataStruct> {
    if transaction_indexed.method.name == "proxyCreateProfile" {
        let meta_raw: LensMeta = LensMeta {
            to: transaction_indexed.method.params[0].value.clone(),
            handle: transaction_indexed.method.params[1].value.clone(),
            imageURI: transaction_indexed.method.params[2].value.clone(),
            followModule: transaction_indexed.method.params[3].value.clone(),
            followModuleInitData: transaction_indexed.method.params[4].value.clone(),
        };

        let mut image;
        if meta_raw.imageURI != "" {
            image = format!("ipfs://{}", meta_raw.imageURI)
        } else {
            image = String::from("https://i.seadn.io/gae/S67RadRtlIbTNk0UojZM-TEl4pybcblKyg3HxQHl0-JmxYZ2deLX-pK2Z89khCWHGeaXeYfE8Vxqj06YCUcqk0q1KWD9T997lGnGHw?auto=format&dpr=1&w=3840")
        }
        let meta_indexed: MetaIndexedStruct = MetaIndexedStruct {
            id: Some(meta_raw.handle.clone()),
            owner: Some(format!("0x{}", meta_raw.to)),
            title: Some(format!("{}.lens", meta_raw.handle.clone())),
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
