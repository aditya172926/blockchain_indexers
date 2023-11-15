// use std::collections::HashMap;

// use crate::helpers::url::helper_url_data;
// use crate::structs::index::MethodParam;
// use crate::structs::meta::{self, Meta, MetaIndexed};
// use crate::structs::transactions::TransactionIndexed;
// use ethers::types::H160;
// use ethers::types::U256;
// use log::{debug, error, info, warn};

// #[derive(Debug)]
// struct LensFollowMeta {
//     profileIds: U256,
//     datas: String,
// }
// #[derive(Debug)]
// struct LensCreateMeta {
//     owner: H160,
//     receiver: H160,
//     cid: String,
// }

// pub async fn handler_lens_post(transaction_indexed: &TransactionIndexed) -> Option<MetaIndexed> {
//     if transaction_indexed.method.name == "post" {
//     } else if transaction_indexed.method.name == "createProfile" {
//         let params_list = transaction_indexed.method.params[0]
//             .clone()
//             .into_tuple()
//             .unwrap();
//         let meta_raw: LensCreateMeta = LensCreateMeta {
//             owner: params_list[0].clone().into_address().unwrap(),
//             receiver: params_list[1].clone().into_address().unwrap(),
//             cid: params_list[2].clone().to_string(),
//         };
//         let raw_data = HashMap::from([
//             (String::from("owner"), meta_raw.owner.to_string()),
//             ((String::from("receiver"), meta_raw.receiver.to_string())),
//             (String::from("cid"), meta_raw.cid),
//         ]);

//         // info!("meta_raw -> {:?}", &meta_raw);

//         let meta_data: MetaData = MetaData {
//             modified: None,
//             raw: raw_data,
//         };
//         return Some(meta_data);
//     } else {
//         return None;
//     }
// }
