use std::collections::HashMap;

use crate::structs::transactions::{TransactionEvent, TransactionIndexed};
use log::info;

pub async fn handler_event_create_sound(
    meta_raw: &mut HashMap<String, String>,
    transaction_event: &TransactionEvent,
) {
    info!("params : {:?}", transaction_event.params);

    // transaction_indexed.event.params[0];
    //     meta_raw: SoundMetaEvent = SoundMetaEvent {
    //     edition: transaction_indexed.event.params[0],
    // };
    //     mut image = String::from(
    //     "https://pbs.twimg.com/profile_images/1455381288756695041/acatxTm8_400x400.jpg",
    // );
    //     meta: Meta = Meta {
    //     id: Some(meta_raw.name.clone()),
    //     owner: Some(meta_raw.owner),
    //     title: Some(format!("{}.eth", meta_raw.name.clone())),
    //     image: Some(image),
    // };
    // println!("\n\n\nMeta indexed {:?} \n\n\n", meta_indexed);
    // let meta_data: MetaDataStruct = MetaData { modified: meta };
    //     return Some(meta_data); tStruct = MetaData { modified: meta };
    //      return None;
    // }
    //     return Some(meta_data); tStruct = MetaData { modified: meta };
    //      return None;
    // }
}
