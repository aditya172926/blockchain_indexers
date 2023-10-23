

// use crate::structs::index::{MetaSchemaAbstractor,Meta, MetaSchema, MetaSource};
// use crate::structs::transactions::TransactionIndexed;
// use crate::utils::contracts::utils_contract_abi;
// use crate::utils::{
//     index::utils_url_data
// };
// use crate::db::index::{db_metaschema_data,db_metaschema_update,db_meta_store};
// // use abstractorutils;
// use ethers::{
//     abi::Abi,
//     contract::Contract,
//     providers::{Http, Provider},
//     types::Address,
// };
// use futures::stream::StreamExt;
// use log::{debug, error, info, warn};
// use mongodb::bson::{Bson, Document};
// use std::collections::HashMap;
// use std::process::exit;
// use std::u128;
// use std::{convert::TryFrom, sync::Arc};


// pub async fn create_meta(meta_slug: &str, event_doc:TransactionIndexed) {
//     println!("Entered create_meta now");
//     // should contain at least 100 txns from current_block_no before starting the code.
//     let meta_schema_result: Option<Document> = db_metaschema_data(meta_slug).await;
//     let meta_schema = match meta_schema_result {
//         Some(result) => result,
//         None => {
//             warn!("No meta_schema found\n");
//             Document::new()
//         }
//     };

//     if meta_schema.is_empty() == true {
//         println!("No document found for the meta_slug\n");
//     } else {
//         let mut chain_id = meta_schema
//             .get_document("contracts")
//             .unwrap()
//             .get_document("contract")
//             .unwrap()
//             .get("chain_id")
//             .unwrap()
//             .to_string();

//         let mut contract_address = meta_schema
//             .get_document("contracts")
//             .unwrap()
//             .get_document("contract")
//             .unwrap()
//             .get("address")
//             .unwrap()
//             .to_string();

//         let read_abi_from_promise = meta_schema
//             .get_document("contracts")
//             .unwrap()
//             .get_document("contract");

//         let mut read_abi_from = match read_abi_from_promise {
//             Ok(object) => match object.get("read_abi_from") {
//                 Some(address) => address.to_string(),
//                 None => {
//                     warn!("Returning empty string, read_abi_from got None\n");
//                     String::new()
//                 }
//             },
//             Err(error) => {
//                 error!("Error in reading field read_abi_from {:?}\n", error);
//                 String::new()
//             }
//         };

//         chain_id = chain_id[1..chain_id.len() - 1].to_string();
//         contract_address = contract_address[1..contract_address.len() - 1].to_string();
//         read_abi_from = read_abi_from[1..read_abi_from.len() - 1].to_string();

//         info!("\n\n the chain id is - {:?}", chain_id);

//         ///////////////////////////////////// creating contract instance START ////////////////////////////////////

//         let mut contract_abi: String;
//         contract_abi = utils_contract_abi(contract_m).await;

//         let abi: Abi = serde_json::from_str(&contract_abi).expect("Error in reading abi json");
//         // println!("\n\n The contract ABI is {:?}", abi);

//         let contract_address_h256 = contract_address
//             .parse::<Address>()
//             .expect("Error in parsing contract address");
//         info!(
//             "The formatted contract address is {:?}\n",
//             contract_address_h256
//         );

//         let provider = Provider::<Http>::try_from(
//             "https://mainnet.infura.io/v3/d0ff55026e4f4547b9f334497888fc07",
//         )
//         .unwrap();
//         // println!("The provider is  {:?}", provider);
//         let contract_instance = Contract::new(contract_address_h256, abi, Arc::new(provider));
//         // println!("The contract instance is {:?}", contract_instance);

//         ///////////////////////////////////// creating contract instance END////////////////////////////////////

//         let loaded_meta_schema: MetaSchemaAbstractor =
//             mongodb::bson::from_bson(Bson::Document(meta_schema)).unwrap();

//         info!("The meta reference is {:?}\n", loaded_meta_schema.reference);
//         let ipfs = match loaded_meta_schema.reference.ipfs {
//             Some(url) => url,
//             None => String::new(),
//         };

//         let erc721_module = match loaded_meta_schema.reference.erc721_module {
//             Some(value) => value,
//             None => String::new(),
//         };

//         // meta contract loop start
//         for element in loaded_meta_schema.source {
//             let contract: String = element.contract;
//             let method: String = element.method;
//             let action_type: String = element.action_type;
//             let block_number: u64 = element.last_block_number;
//             // let block_number: u64 = 17978560;
//             let stop_block_number: u64 = block_number + 50000;
//             // let stop_block_number: i64 = 17808560; //close attention to this 45584707
//             info!(
//                 "block number before -> {} and stop_block_number -> {}\n\n",
//                 block_number, stop_block_number
//             );

//             let mut prop_list: Vec<String> = Vec::new();

//             for prop in element.data {
//                 prop_list.push(prop.prop);
//             }

//             let mut current_block_number: u64 = 0;

//             if current_block_number >= stop_block_number {
//                 exit(1);
//             }
//             println!("this is the txn while loop starting");
//                 let loaded_transaction= event_doc.clone();
                
//                 // TransactionIndexed = match doc { // pass the event_document from the indexer
//                 //     Ok(object) => match mongodb::bson::from_bson(Bson::Document(object)) {
//                 //         Ok(txn_document) => txn_document,
//                 //         Err(error) => {
//                 //             error!(
//                 //                 "Could not convert to Bson documen, error -> {:?}\n\n",
//                 //                 error
//                 //             );
//                 //             if current_block_number > 0 {
//                 //                 info!("Updating database...\n");
//                 //                 let _ =
//                 //                     db::db_metaschema_update(current_block_number, meta_slug).await;
//                 //             } else {
//                 //                 info!("Requires manual intervention; current_block_number value -> {}\n", current_block_number);
//                 //             }
//                 //             println!("Process Exiting...\n");
//                 //             exit(1);
//                 //         }
//                 //     },
//                 //     Err(error) => {
//                 //         error!("Could not decode transaction error -> {:?}\n\n", error);
//                 //         if current_block_number > 0 {
//                 //             info!("Updating database...\n");
//                 //             let _ = db::db_metaschema_update(current_block_number, meta_slug).await;
//                 //         } else {
//                 //             info!(
//                 //                 "Requires manual intervention; current_block_number value -> {}\n",
//                 //                 current_block_number
//                 //             );
//                 //         }
//                 //         println!("Process Exiting...\n");
//                 //         exit(1);
//                 //     }
//                 // };

//             let mut meta_source: Vec<MetaSource> = Vec::new();
//             meta_source.push(MetaSource {
//                 contract: String::from(&contract),
//                 method: String::from(&method),
//                 action_type: String::from(&action_type),
//                 value: format!("0x{:020x}", loaded_transaction.transaction.txn_hash),
//             });

//                  current_block_number=loaded_transaction.transaction.block_number;
//                 //get the current block number
//                 // current_block_number = match loaded_transaction.transaction.block_number {
//                 //     Some(txn_block_number) => txn_block_number,
//                 //     None => {
//                 //         error!("Could not get current_block_number, got None\n\n");
//                 //         if current_block_number > 0 {
//                 //             info!("Updating database...\n");
//                 //             let _ = db_metaschema_update(current_block_number, meta_slug).await;
//                 //         } else {
//                 //             info!(
//                 //                 "Requires manual intervention; current_block_number value -> {}\n",
//                 //                 current_block_number
//                 //             );
//                 //         }
//                 //         println!("Process Exiting...\n");
//                 //         exit(1);
//                 //     }
//                 // };
//                 info!("current block Number: {:?}", current_block_number);

//             let mut metadata_list: HashMap<String, HashMap<String, serde_json::Value>> =
//                 HashMap::new();
//             let mut raw_hashmap: HashMap<String, serde_json::Value> = HashMap::new();

//             let mut meta_id: String = String::new();
//             let mut meta_owner: String = format!("0x{:020x}", loaded_transaction.transaction.from);
//             // let mut meta_owner: String = String::new();

//                 // creating meta loop
//                 for param in loaded_transaction.method.params {
//                     if prop_list.contains(&param.name) {
//                         let param_value = param.value.as_str();

//                         // ipfs module
//                         if ipfs == param.name {
//                             info!("Found a match for {} with the value -> {:?}\n\n", ipfs, param_value);
//                             // get data with https
//                             let response = utils_url_data(param_value).await;
//                             // let response = get(param_value).await.unwrap();

//                         match response {
//                             Ok(object) => {
//                                 if object.status().is_success() {
//                                     let res = object.text().await.expect("Error in parsing object");
//                                     let ipfs_content: serde_json::Value =
//                                         serde_json::from_str(&res)
//                                             .expect("error in reading json format");
//                                     info!("The json body is {:?}\n\n", ipfs_content);
//                                     let metaId = ipfs_content["metadata_id"].to_string();
//                                     meta_id = metaId[1..metaId.len() - 1].to_string();

//                                     let mut ipfs_hashmap: HashMap<String, serde_json::Value> =
//                                         HashMap::new();
//                                     ipfs_hashmap.insert(String::from(&param.name), ipfs_content);
//                                     metadata_list.insert(String::from("ipfs"), ipfs_hashmap);
//                                     // .insert(String::from(&param.name), ipfs_content);
//                                 } else {
//                                     warn!("The response failed\n");
//                                 }
//                             }
//                             Err(error) => {
//                                 error!("Error in fetching response of key -> {:?} value -> {:?}, Error -> {:?}\n\n", param.name, param_value, error);
//                             }
//                         }
//                     }
//                     // erc721 module
//                     else if erc721_module == param.name {
//                         info!(
//                             "Found a match of erc721 module {:?} {:?}\n",
//                             param.name, param_value
//                         );

//                         meta_id = param_value.to_string();
//                         // let token_id = u12param_value;
//                         let token_id = u128::from_str_radix(param_value, 10).unwrap();
//                         info!("The token id is {:?}\n\n", token_id);

//                         let get_token_url =
//                             contract_instance.method::<_, String>("tokenURI", token_id);

//                         let token_url = match get_token_url {
//                             Ok(method) => {
//                                 let token_url_promise = method.call().await;
//                                 match token_url_promise {
//                                     Ok(result) => result,
//                                     Err(e) => {
//                                         error!("Error in contract call -> {:?}\n\n", e);
//                                         String::new()
//                                     }
//                                 }
//                             }
//                             Err(e) => {
//                                 error!("Error in get_token_url method from the smart contract {:?}\n\n", e);
//                                 String::new()
//                             }
//                         };

//                             if !token_url.is_empty() {
//                                 let response = utils_url_data(&token_url).await;
//                                 // let response: Result<reqwest::Response, reqwest::Error> = get(&token_url).await;
//                                 match response {
//                                     Ok(object) => {
//                                         let nft_data = object
//                                             .text()
//                                             .await
//                                             .expect("Error in parsing nft data to string\n");
//                                         info!("The nft data is {:?}\n\n", nft_data);
//                                         let nft_data_content = serde_json::from_str(&nft_data)
//                                             .expect("error in reading json format");
//                                         let mut erc721_data_hashmap: HashMap<
//                                             String,
//                                             serde_json::Value,
//                                         > = HashMap::new();
//                                         erc721_data_hashmap
//                                             .insert(String::from(&param.name), nft_data_content);
//                                         metadata_list.insert(
//                                             "erc721_module".to_string(),
//                                             erc721_data_hashmap,
//                                         );
//                                     }
//                                     Err(error) => {
//                                         error!(
//                                             "Error in fetching nft data from url {:?}\n\n",
//                                             error
//                                         );
//                                     }
//                                 }
//                             }
//                             info!("The token url is {:?}\n\n", token_url);
//                         }

//                     raw_hashmap.insert(
//                         String::from(&param.name),
//                         serde_json::Value::String(param.value),
//                     );
//                 }
//             }
//             metadata_list.insert(String::from("raw"), raw_hashmap);

//             info!(
//                 "The meta raw list element -------------> {:?}\n",
//                 metadata_list
//             );

//             // println!("\n\n################ The meta is {:?}", meta);
//             // start here the creation of modified data
//             let mut modified_list: HashMap<String, serde_json::Value> = HashMap::new();

//             for obj in &loaded_meta_schema.data {
//                 let option_tree = &obj.prop_field;
//                 let mut tree: Vec<&str> = Vec::new();
//                 if !option_tree.is_none() {
//                     tree = option_tree.as_ref().unwrap().split("|").collect();
//                 }
//                 info!("The tree is {:?}\n", tree);
//                 let mut json_value: serde_json::Value;
//                 if tree.len() > 0 {
//                     // let current_value = metadata_list.get(tree[0]);
//                     match metadata_list.get(tree[0]) {
//                         // can be the area of crash for lens profile
//                         Some(current_value) => {
//                             json_value =
//                                 serde_json::json!(current_value.get(tree[1]).unwrap().clone());
//                             let mut tree_index = 2;

//                                 while tree_index < tree.len() {
//                                     json_value = json_value[tree[tree_index]].clone();
//                                     if json_value.is_null() {
//                                         // json_value = obj.prop_default.as_ref().unwrap().clone();
//                                         json_value = match obj.prop_default.as_ref() {
//                                             Some(object) => object.clone(),
//                                             None => {
//                                                 warn!("Json_value of metadata_list returned None, json_value -> {:?}\n", json_value);
//                                                 info!("Breaking the loop...\n");
//                                                 break;
//                                                 // info!("Updating database...\n");
//                                                 // // exit the program and update the last block number on meta_schema
//                                                 // let _ = db::db_metaschema_update(
//                                                 //     current_block_number,
//                                                 //     meta_slug,
//                                                 // )
//                                                 // .await;
//                                                 // info!("Process exiting...\n");
//                                                 // exit(1); // exiting the code
//                                             }
//                                         };
//                                     }
//                                     tree_index += 1;
//                                 }
//                                 info!("Json value %%%%% {:?}\n", json_value);
//                                 if !json_value.is_null() {
//                                     modified_list.insert(String::from(&obj.prop), json_value);
//                                 }
//                             }
//                             None => {
//                                 error!(
//                                     "Error in finding key {:?} Or the url returned None",
//                                     tree[0]
//                                 );
//                                 continue;
//                                 // if current_block_number > 0 {
//                                 //     info!("Updating database...\n");
//                                 //     let _ =
//                                 //         db::db_metaschema_update(current_block_number, meta_slug)
//                                 //             .await;
//                                 // } else {
//                                 //     info!("Requires manual intervention; current_block_number value -> {}\n", current_block_number);
//                                 // }
//                                 // info!("Process Exiting...\n");
//                                 // exit(1);
//                             }
//                         }; // possible area of crash
//                     } else {
//                         let option_json_value = &obj.prop_default;
//                         if option_json_value.is_some() {
//                             json_value = option_json_value.as_ref().unwrap().clone();
//                             println!("Json value %%%%% {:?}\n", json_value);
//                             modified_list.insert(String::from(&obj.prop), json_value);
//                         }
//                     }
//                 }

//             if !modified_list.is_empty() {
//                 metadata_list.insert(String::from("modified"), modified_list);

//                     let meta: Meta = Meta {
//                         slug: meta_slug.to_string(),
//                         data: metadata_list,
//                         sources: meta_source,
//                         indexable: true,
//                     };

//                     println!("The meta is {:?}\n\n", meta);
//                     let _ = db_meta_store(meta, meta_id, meta_owner).await;
//                     println!("===============================================================================================\n");
//                 }

//             // index += 1;
//             // }
//             info!("after: {}\n", current_block_number);
//             info!("Updating database...");
//             if current_block_number == 0 {
//                 let _ = db_metaschema_update(stop_block_number, meta_slug).await;
//             } else {
//                 let _ = db_metaschema_update(current_block_number, meta_slug).await;
//             }
//             info!("All the transaction logs between {} and {} are abstracted into metas.\nStart the node again to continue from the latest block number", block_number, stop_block_number);
//         }
//     }
// }
