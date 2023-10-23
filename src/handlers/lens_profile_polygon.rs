use ethers::types::H160;
use log::{debug, error, info, warn};
use crate::structs::meta::{self, Meta, MetaData};
use crate::structs::transactions::TransactionIndexed;
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
        let params_list = transaction_indexed.method.params[0].clone().into_tuple().unwrap();
        info!("params_list -> {:?}", params_list);
        let meta_raw: LensMeta = LensMeta {
            to: params_list[0].clone().into_address().unwrap(),
            handle: params_list[1].to_string(),
            imageURI: params_list[2].to_string(),
            followModule: params_list[3].to_string(),
            followModuleInitData: params_list[4].to_string(),
        };

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
        };
        let meta_data: MetaData = MetaData { modified: meta };
        return Some(meta_data);
    } else {
        return None;
    }
}
