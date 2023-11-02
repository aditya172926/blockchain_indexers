use web3::contract::ens::Ens;

use crate::structs::index::{Meta, MethodParam};
use crate::structs::meta::{self, MetaDataStruct, MetaIndexedStruct};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct SoundMeta {
    salt: String,
    initData: Vec<String>,
    contracts: Vec<H160>,
    data: String
}

#[derive(Debug)]
struct SoundMetaEvent {
    edition: H160,
    name:String,
    symbol:String,
    metadataModule: H160,
    baseURI: String,
    contractURI:String,
    fundingRecipient:H160,
    royaltyBPS: String,
    editionMaxMintableLower:String,
    editionMaxMintableUpper:String,
    editionCutoffTime:String,
    flags:String
}

pub fn handler_sound(transaction_indexed: &TransactionIndexed) -> Option<MetaDataStruct> {
    if transaction_indexed.method.name == "createSoundAndMints" {
        let meta_raw: SoundMeta = SoundMeta {
            salt: transaction_indexed.method.params[0].value.clone(),
            initData: transaction_indexed.method.params[1].value.clone(),
            contracts: transaction_indexed.method.params[2].value.clone(),
            data: transaction_indexed.method.params[3].value.clone(),
        };
        let mut image = String::from(
            "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg",
        );
        let meta: Meta = Meta {
            id: Some(meta_raw.name.clone()),
            owner: Some(meta_raw.owner),
            title: Some(format!("{}.eth", meta_raw.name.clone())),
            image: Some(image),
        };
        println!("\n\n\nMeta indexed {:?} \n\n\n", meta_indexed);
        let meta_data: MetaDataStruct = MetaData { modified: meta };
        return Some(meta_data);
    } else {
        return None;
    }
}

pub fn event_handler_sound(transaction_indexed: &TransactionIndexed) -> Option<MetaDataStruct> {
    if transaction_indexed.event.name == "SoundEditionInitialized" {
        let t=transaction_indexed.event.params[0];
        let meta_raw: SoundMetaEvent = SoundMetaEvent {
            edition: transaction_indexed.event.params[0],
        };
        let mut image = String::from(
            "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg",
        );
        let meta: Meta = Meta {
            id: Some(meta_raw.name.clone()),
            owner: Some(meta_raw.owner),
            title: Some(format!("{}.eth", meta_raw.name.clone())),
            image: Some(image),
        };
        println!("\n\n\nMeta indexed {:?} \n\n\n", meta_indexed);
        let meta_data: MetaDataStruct = MetaData { modified: meta };
        return Some(meta_data);
    } else {
        return None;
    }
}
