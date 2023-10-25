
use std::process::exit;

use log::{debug, error, info, warn};

use crate::{
    handlers::{ens_ethereum::handler_ens, lens_post::handler_lens_post, lens_profile_polygon::handler_lens_profile},
    structs::{extract::Schema, meta::MetaIndexed, transactions::TransactionIndexed},
};



pub async fn utils_meta_indexed(
    schema: &Schema,
    transaction_indexed: TransactionIndexed,
) -> Option<MetaIndexed> {
    let fname=&schema.source[0].handlersMethods;
    print!("the fname: {}",fname);

    // exit(1);
    let mut meta_indexed: MetaIndexed=MetaIndexed::default();
    match fname.as_ref(){
        "handler_ens"=>{
            match handler_ens(&transaction_indexed).await {
                Some(object) => {
                     meta_indexed = MetaIndexed {
                        owner: object.clone().modified.unwrap().owner.unwrap(),
                        id: object.clone().modified.unwrap().id.unwrap(),
                        slug: schema.slug.clone(),
                        data: object.clone(),
                        createdAt: String::from(""),
                        updatedAt: String::from(""),
                        sources: vec![transaction_indexed],
                    };
                    Some(meta_indexed)
                }
                None => {
                    warn!("handler returned null");
                    None
                }
            }
        },
        "handler_lens_post"=>{
            match handler_lens_post(&transaction_indexed).await {
                Some(object) => {
                     meta_indexed = MetaIndexed {
                        owner: object.clone().modified.unwrap().owner.unwrap(),
                        id: object.clone().modified.unwrap().id.unwrap(),
                        slug: schema.slug.clone(),
                        data: object.clone(),
                        createdAt: String::from(""),
                        updatedAt: String::from(""),
                        sources: vec![transaction_indexed],
                    };
                    Some(meta_indexed)
                }
                None => {
                    warn!("handler returned null");
                    None
                }
            }
        },
        "handler_lens_profile"=>{
            match handler_lens_profile(&transaction_indexed).await {
                Some(object) => {
                     meta_indexed = MetaIndexed {
                        owner: object.clone().modified.unwrap().owner.unwrap(),
                        id: object.clone().modified.unwrap().id.unwrap(),
                        slug: schema.slug.clone(),
                        data: object.clone(),
                        createdAt: String::from(""),
                        updatedAt: String::from(""),
                        sources: vec![transaction_indexed],
                    };
                    Some(meta_indexed)
                }
                None => {
                    warn!("handler returned null");
                    None
                }
            }
        },
        _=>{
            return Some(meta_indexed)
        }
    }
    // let meta_indexed: Option<MetaIndexed> = 
    // info!("\nmeta indexed {:?}\n", meta_indexed);
    // meta_indexed

    
}
