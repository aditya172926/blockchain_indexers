// use crate::structs::{ContractData, MethodParam, Transaction, Meta};
use crate::structs::contracts::ContractData;
use crate::structs::extract::Db;
use crate::structs::meta::MetaIndexed;
use crate::structs::{self, index::MethodParam, transactions::TransactionIndexed};
use chrono::Utc;
use ethcontract::RawLog;
use ethers::types::TransactionReceipt;
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

pub async fn db_meta_store(db: &Db, meta: MetaIndexed) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse(db.client.clone()).await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database(&db.database);
    let collection: mongodb::Collection<Document> = db.collection::<Document>("metas");
    let doc_bson: Bson = to_bson(&meta).unwrap();
    let doc = doc! {"document":doc_bson};
    collection.insert_one(doc, None).await?;
    info!("Inserted meta doc");

    Ok(())
}
