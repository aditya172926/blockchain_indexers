use std::collections::HashMap;

use crate::structs::contracts::ContractIndexed;
use crate::structs::extract::Schema;
use crate::structs::meta::{self, Meta, MetaIndexed, MetaResult};
use crate::structs::transactions::TransactionIndexed;
use base64::Engine;
use base64::engine::general_purpose;
use ethers::types::H160;
use log::{debug, error, info, warn};
use crate::handlers::lens_profile_polygon::handler;



#[derive(Debug)]
struct LensMeta {
    to: H160,
    handle: String,
    imageURI: String,
    followModule: String,
    followModuleInitData: String,
    //missing data: followNFTURI
}

#[derive(Debug)]
struct LensProfileMeta {
    owner:H160,
    tokenId: String,
}


pub async fn handler_lens_profile(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
    contracts: &mut Vec<ContractIndexed>
) -> Option<MetaResult> {
    match &transaction_indexed.method {
        Some(transaction_indexed_method) => {
            if transaction_indexed_method.name == "proxyCreateProfile" {
                return handler::handler_txn_create_profile_lens(transaction_indexed,schema).await;
            } else if transaction_indexed_method.name == "setProfileImageURI" {
                return handler::handler_txn_set_profile_lens(transaction_indexed,schema).await;
            } else {
                return None;
            }
        },
        None=>{
            let mut create_contract=contracts[0].instance.clone();
            let mut meta_raw: HashMap<String, String> = HashMap::new();
            for contract in contracts{
                if contract.instance.address()=="0xDb46d1Dc155634FbC732f92E853b10B288AD5a1d".parse::<H160>().unwrap(){
                    create_contract=contract.instance.clone();
                }
            }
            for event in transaction_indexed.events.as_ref().unwrap() {
                if format!("0x{:x}", event.topic0)
                == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
                    {
                        let metadata=create_contract.method::<_,String>("tokenURI", event.params[2].clone()).unwrap().call().await.unwrap();
                        
                        let metadata_split:Vec<&str>=metadata.split("64,").collect();
                        let data=metadata_split[1].clone();
                            
                        let decoded_metadata=general_purpose::STANDARD.decode(data).unwrap();
                        let str_metadata = std::str::from_utf8(&decoded_metadata).unwrap();

                        let metadata_obj:serde_json::Value=serde_json::from_str(str_metadata).unwrap();

                        // println!("THIS IS IT:///////////////{:?}",metadata_obj);

                    return handler::handler_event_create_profile(metadata_obj,transaction_indexed,event,schema).await;
                    // return None;
                    };
            }            
            None
        }
    }
}
