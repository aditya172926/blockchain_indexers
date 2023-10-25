use std::collections::HashMap;

use crate::structs::meta::{self, Meta, MetaData};
use crate::structs::transactions::TransactionIndexed;
use ethers::types::H160;
use log::{debug, error, info, warn};
#[derive(Debug)]
struct LensMeta {
    to: H160,
    handle: String,
    imageURI: String,
    followModule: String,
    followModuleInitData: String,
    //missing data: followNFTURI
}

pub async fn handler_lens_profile(transaction_indexed: &TransactionIndexed) -> Option<MetaData> {
    if transaction_indexed.method.name == "proxyCreateProfile" {
        let params_list = transaction_indexed.method.params[0]
            .clone()
            .into_tuple()
            .unwrap();
        info!("params_list -> {:?}", params_list);
        let meta_raw: LensMeta = LensMeta {
            to: params_list[0].clone().into_address().unwrap(),
            handle: params_list[1].to_string(),
            imageURI: params_list[2].to_string(),
            followModule: params_list[3].to_string(),
            followModuleInitData: params_list[4].to_string(),
        };
        let raw_data = HashMap::from([
            (String::from("to"), meta_raw.to.to_string()),
            ((String::from("handle"), meta_raw.handle.to_owned())),
            (String::from("imageURI"), meta_raw.imageURI.to_owned()),
            (String::from("followModule"), meta_raw.followModule),
            (
                String::from("followModuleInitData"),
                meta_raw.followModuleInitData,
            ),
        ]);

        let mut image;
        if meta_raw.imageURI != "" {
            image = format!("ipfs://{}", meta_raw.imageURI)
        } else {
            image = String::from("https://i.seadn.io/gae/S67RadRtlIbTNk0UojZM-TEl4pybcblKyg3HxQHl0-JmxYZ2deLX-pK2Z89khCWHGeaXeYfE8Vxqj06YCUcqk0q1KWD9T997lGnGHw?auto=format&dpr=1&w=3840")
        }
        let meta: Meta = Meta {
            id: Some(meta_raw.handle.clone()),
            owner: Some(meta_raw.to),
            title: Some(format!("{}.lens", meta_raw.handle.clone())),
            image: Some(image),
            content: None,
        };
        let meta_data: MetaData = MetaData {
            modified: Some(meta),
            raw: raw_data,
        };
        return Some(meta_data);
    } else {
        return None;
    }
}
