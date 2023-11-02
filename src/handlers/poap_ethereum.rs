// use std::collections::HashMap;

// use crate::structs::extract::Schema;
// use crate::structs::meta::{self, Meta, MetaIndexed, MetaResult};
// use crate::structs::transactions::TransactionIndexed;
// use ethers::types::{H160, U128, U256};
// use log::{debug, error, info, warn};

// #[derive(Debug)]
// struct PoapMeta {
//     from: H160,
//     to: H160,
//     tokenId: String,
// }

// pub async fn handler(from: H160, to: H160, tokenId: String, slug: String) -> Option<MetaIndexed> {
//     let meta_raw: PoapMeta = PoapMeta {
//         from: from,
//         to: to,
//         tokenId: tokenId.clone(),
//     };
//     let mut raw_data: HashMap<String, String> = HashMap::from([
//         (String::from("from"), format!("0x{:x}", meta_raw.from)),
//         (String::from("to"), format!("0x{:x}", meta_raw.to)),
//         (String::from("tokenId"), meta_raw.tokenId.clone()),
//     ]);
//     let mut image = String::new();

//     let meta_modified: Meta = Meta {
//         id: Some(meta_raw.tokenId.clone()),
//         owner: Some(meta_raw.to),
//         title: Some(meta_raw.tokenId.clone()),
//         image: Some(image),
//         content: None,
//     };
//     let meta_indexed = MetaIndexed {
//         owner: to,
//         id: tokenId,
//         slug,
//         raw: raw_data,
//         modified: Some(meta_modified),
//         createdAt: "".to_string(),
//         updatedAt: "".to_string(),
//     };

//     // println!("\n\n\nMeta indexed {:?} \n\n\n", meta);

//     return Some(meta_indexed);
// }

// pub async fn handler_poap_events(
//     transaction_indexed: &TransactionIndexed,
//     schema: &Schema,
// ) -> Option<MetaResult> {
//     let transaction_indexed_event = transaction_indexed.event.clone().unwrap();
//     if transaction_indexed_event.name == "Transfer" {
//         let meta_data = handler(
//             transaction_indexed_event.params[0]
//                 .clone()
//                 .into_address()
//                 .unwrap(),
//             transaction_indexed_event.params[1]
//                 .clone()
//                 .into_address()
//                 .unwrap(),
//             transaction_indexed_event.params[2]
//                 .clone()
//                 .into_uint()
//                 .unwrap()
//                 .to_string(),
//             schema.slug.clone(),
//         )
//         .await;

//         let result: MetaResult = MetaResult {
//             id: transaction_indexed_event.params[2].to_string(),
//             owner: transaction_indexed_event.params[1].to_string(),
//             slug: schema.slug.clone(),
//             insert: meta_data,
//             update: None,
//             source: transaction_indexed.clone(),
//         };
//         info!("result from handler poap  :  {:?}", result);
//         return Some(result);
//     } else {
//         None
//     }
// }

// pub async fn handler_poap_ethereum(
//     transaction_indexed: &TransactionIndexed,
//     schema: &Schema,
// ) -> Option<MetaResult> {
//     let transaction_indexed_method = transaction_indexed.method.clone().unwrap();
//     if transaction_indexed_method.name == "transferFrom"
//         || transaction_indexed_method.name == "safeTransferFrom"
//     {
//         let meta_data = handler(
//             transaction_indexed_method.params[0]
//                 .clone()
//                 .into_address()
//                 .unwrap(),
//             transaction_indexed_method.params[1]
//                 .clone()
//                 .into_address()
//                 .unwrap(),
//             transaction_indexed_method.params[2]
//                 .clone()
//                 .into_uint()
//                 .unwrap()
//                 .to_string(),
//             schema.slug.clone(),
//         )
//         .await;

//         let result: MetaResult = MetaResult {
//             id: transaction_indexed_method.params[2].to_string(),
//             owner: transaction_indexed_method.params[1].to_string(),
//             slug: schema.slug.clone(),
//             insert: meta_data,
//             update: None,
//             source: transaction_indexed.clone(),
//         };
//         // info!("result from handler poap  :  {:?}", result);
//         return Some(result);
//     } else {
//         None
//     }
// }
