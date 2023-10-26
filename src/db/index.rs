use std::collections::HashMap;

// use crate::structs::{ContractData, MethodParam, Transaction, Meta};
use crate::structs::contracts::ContractData;
use crate::structs::extract::Db;
use crate::structs::meta::{MetaIndexed, MetaResult};
use crate::structs::{self, index::MethodParam, transactions::TransactionIndexed, log::Log};
use chrono::Utc;
use ethcontract::RawLog;
use ethers::types::TransactionReceipt;
use futures::future::ok;
use futures::{StreamExt, TryStreamExt};
use log::info;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    error::Error,
    options::ClientOptions,
    Client, Cursor,
};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Serialize)]
struct BytesWrapper<'a> {
    #[serde(serialize_with = "serialize_bytes")]
    bytes: &'a [u8],
}

fn serialize_bytes<S>(bytes: &&[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bson_array = bytes
        .iter()
        .map(|&byte| Bson::Int32(byte as i32))
        .collect::<Vec<Bson>>();
    serializer.serialize_some(&bson_array)
}

pub async fn db_log_store(db:&Db,log:Log)-> Result<(), Box<dyn std::error::Error>>{
    let client_options: ClientOptions = ClientOptions::parse(db.client.clone()).await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database(&db.database);
    let collection: mongodb::Collection<Document> = db.collection::<Document>("logs");
    let bson_log=to_bson(&log).unwrap();
    let doc = doc! {"log":bson_log};

    collection.insert_one(doc, None).await;
    Ok(())
}

pub async fn db_meta_store(
    db: &Db,
    results: Vec<MetaResult>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse(db.client.clone()).await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database(&db.database);
    let collection: mongodb::Collection<Document> = db.collection::<Document>("metas");

    for result in results {
        match result.insert {
            Some(object) => {
                let filter = doc! {
                    "sources.transaction.txn_hash": format!("0x{:x}", &result.source.transaction.txn_hash),
                    "sources.transaction.chain_id": Bson::Int64(result.source.transaction.chain_id as i64),
                    "document.slug":&object.slug,
                    "document.id":&object.id
                };
                if collection.find_one(filter, None).await?.is_none() {
                    let object_bson: Bson = to_bson(&object).unwrap();
                    let source = vec![result.source];
                    let source_bson: Bson = to_bson(&source).unwrap();
                    let doc = doc! {"document":object_bson, "sources":source_bson};

                    // Insert the document if it doesn't exist
                    collection.insert_one(doc, None).await?;
                } else {
                    info!("Meta document already exists in the database");
                }
            }
            None => {
                let filter = doc! {
                    "sources.transaction.chain_id": Bson::Int64(result.source.transaction.chain_id as i64),
                    "document.slug":result.slug,
                    "document.id":&result.source.method.params[0].to_string()
                };

                // for (key, value) in result.update.unwrap().into_iter() {
                //     let update = doc! {"$set": {key:value}};
                // }
                let source = vec![result.source];
                let source_bson: Bson = to_bson(&source).unwrap();
                let update = doc! {"$set": to_bson(&result.update).unwrap(),"$push":{"sources":source_bson}};

                collection.update_one(filter, update, None).await.unwrap();
            }
        }
    }

    // for meta_item in result {
    // if let Some(source) = meta_item.sources.last() {
    //     // println!("\n\n\n {} \n\n\n", )
    //     let filter = doc! {
    //         "document.sources.transaction.txn_hash": format!("0x{:x}", &source.transaction.txn_hash),
    //         "document.sources.transaction.chain_id": Bson::Int64(source.transaction.chain_id as i64),
    //     };

    //     // Check if a document with the same slug and title already exists
    //     if collection.find_one(filter, None).await?.is_none() {
    //         let doc_bson: Bson = to_bson(&meta_item).unwrap();
    //         let doc = doc! {"document": doc_bson};

    //         // Insert the document if it doesn't exist
    //         collection.insert_one(doc, None).await?;
    //         info!("Inserted meta doc: {:?}", meta_item);
    //     } else {
    //         info!("Meta document already exists in the database");
    //     }
    // }
    // }

    Ok(())
}
