use std::collections::HashMap;

use crate::structs::meta::{self, Meta};
use crate::structs::transactions::TransactionIndexed;
use ethers::types::{H160, U128, U256};
use log::{debug, error, info, warn};

#[derive(Debug)]
struct PoapMeta {
    from: H160,
    to: H160,
    tokenId: String,
}

// pub async fn handler_poap_ethereum(transaction_indexed: &TransactionIndexed) -> Option<MetaData> {
//     if transaction_indexed.method.name == "transferFrom"
//         || transaction_indexed.method.name == "safeTransferFrom"
//     {
//         let meta_raw: PoapMeta = PoapMeta {
//             from: transaction_indexed.method.params[0]
//                 .clone()
//                 .into_address()
//                 .unwrap(),
//             to: transaction_indexed.method.params[1]
//                 .clone()
//                 .into_address()
//                 .unwrap(),
//             tokenId: transaction_indexed.method.params[2]
//                 .clone()
//                 .into_uint()
//                 .unwrap()
//                 .to_string(),
//         };

//         let raw_data = HashMap::from([
//             (String::from("profileId"), meta_raw.from.to_string()),
//             ((String::from("contentURI"), meta_raw.to.to_string())),
//             (String::from("collectModule"), meta_raw.tokenId.to_string()),
//         ]);

//         info!("meta_raw -> {:?}\n", meta_raw);

//         let mut image = String::new();

//         let meta: Meta = Meta {
//             id: Some(meta_raw.tokenId.clone()),
//             owner: Some(meta_raw.to),
//             title: Some(meta_raw.tokenId),
//             image: Some(image),
//             content: None,
//         };
//         let meta_data: MetaData = MetaData {
//             modified: Some(meta),
//             raw: raw_data,
//         };
//         Some(meta_data)
//     } else {
//         None
//     }
// }
