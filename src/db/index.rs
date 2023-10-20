// use crate::structs::{ContractData, MethodParam, Transaction, Meta};
use ethcontract::RawLog;
use ethers::types::TransactionReceipt;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    error::Error,
    options::ClientOptions,
    Client, Cursor,
};
use futures::{StreamExt, TryStreamExt};
use chrono::Utc;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use crate::structs::{index::{MethodParam}, self, transactions::TransactionIndexed};
use crate::structs::contracts::ContractData;

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

pub async fn db_contract_data(contract_slug: &str) -> Option<Document> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await.unwrap();
    let client: Client = Client::with_options(client_options).unwrap();
    let db: mongodb::Database = client.database("macha_dev"); // reading contract data
    let collection: mongodb::Collection<Document> = db.collection::<Document>("contracts");

    let result: Option<Document> = collection
        .find_one(doc! {"contract.slug": contract_slug}, None)
        .await
        .unwrap();

    // match &result {
    //      Some(doc) => println!("The document result is {:?}", doc),
    //      None => println!("No document found")
    // };
    return result;
}

pub async fn db_event_store(event: RawLog) -> Result<(), Box<dyn std::error::Error>> {
    let mut json_object = json!({});
    json_object["topics"] = serde_json::to_value(&event.topics).unwrap();
    let data_wrapper = BytesWrapper { bytes: &event.data };
    let bson_data: Bson = to_bson(&data_wrapper).unwrap();
    println!("The bson data is {:?}", bson_data);
    json_object["data"] = Value::from(event.data);

    let client_options = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("macha_dev"); // writing events to db
    let collection = db.collection::<Document>("events");

    let event_bson: mongodb::bson::Bson = to_bson(&json_object).unwrap();


    let event_document = doc! {
        "event": event_bson,
        "timestamp": "Testingt time stamp",
    };

    println!("The event document is {:?}", event_document);

    collection.insert_one(event_document, None).await?;

    println!("Event document inserted successfully!");

    Ok(())
}

pub async fn db_transaction_store(
    transaction_indexed:TransactionIndexed
) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("macha_dev"); // writing transactions to db
    let collection: mongodb::Collection<Document> = db.collection::<Document>("transactions");
    let event_doc = doc! {
            "transaction": {},
            "timestamp": "Testingt time stamp",
        };
    
    
    // pass the event document to abstractor loaded_transaction
    collection.insert_one(event_doc, None).await?;
    println!("Event document inserted successfully!");
    Ok(())
}

pub async fn db_contract_store(
    contract_data: ContractData,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("macha_dev"); // saving contracts to db
    let collection: mongodb::Collection<Document> = db.collection::<Document>("contracts");

    let result = collection.find(doc! {"contract_name": &contract_data.name}, None);
    // if (result)
    let contract_bson: mongodb::bson::Bson = to_bson(&contract_data).unwrap();
    let contract_document = doc! {
        "contract": contract_bson,
        "timestamp": "Testingt time stamp",
    };

    collection.insert_one(contract_document, None).await?;
    println!("The contract document is inserted");

    Ok(())
}

pub async fn db_metaschema_data(meta_slug: &str) -> Option<Document> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db: mongodb::Database = client.database("macha_dev");
    let collection: mongodb::Collection<Document> = db.collection::<Document>("metas_schemas");

    let meta_schema_pipeline = vec![
        doc! {"$match": {
            "contract_slug": meta_slug
        }},
        doc! {"$lookup": {
            "from": "contracts",
            "localField": "contract_slug",
            "foreignField": "contract.slug",
            "as": "contracts"
        }},
        doc! {"$unwind": "$contracts"},
        doc! {"$project": {
            "contracts.contract.name": false,
            "contracts.contract.description": false,
            "contracts.contract.interested_methods": false,
            "contracts.contract.interested_events": false,
            "contracts.contract.is_approved": false
        }},
    ];
    // println!("The meta schema pipeline {:?}", meta_schema_pipeline);
    let meta_schama_result = collection.aggregate(meta_schema_pipeline, None).await;

    match meta_schama_result {
        Ok(mut result) => {
            // println!("The meta schema fetched is {:?}", result);
            let mut response = Some(Document::new());
            while let Some(obj) = result.try_next().await.unwrap() {
                // println!("The object from cursor {:?}", obj);
                response = Some(obj);
            }
            response
        }
        Err(e) => {
            println!("Error in fetching meta_schema {:?}", e);
            None
        }
    }
}

pub async fn db_metaschema_update(
    block_number: u64,
    meta_slug: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("macha_dev");
    let collection: mongodb::Collection<Document> = db.collection::<Document>("metas_schemas");

    // let block_number: Bson = to_bson(&meta).unwrap();
    let meta_schema_doc = doc! {"slug": meta_slug, "source.meta_slug": meta_slug};
    let update_block_query = doc! {"$set": {"source.$.last_block_number": block_number as i64}};
    let update_result = collection
        .update_one(meta_schema_doc, update_block_query, None)
        .await?;
    println!("The update result is {:?}", update_result);

    Ok(())
}

pub async fn db_meta_store(
    meta: structs::index::Meta,
    meta_id: String,
    meta_owner: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("macha_dev");
    let collection: mongodb::Collection<Document> = db.collection::<Document>("metas");

    let meta_bson: Bson = to_bson(&meta).unwrap();
    let now = Utc::now();
    let ts: String = now.timestamp().to_string();
    println!("Current timestamp is: {}", ts);
    let meta_doc =
        doc! {"metaId": meta_id, "metaOwner": meta_owner, "meta": meta_bson, "timestamp": ts};
    collection.insert_one(meta_doc, None).await?;
    println!("Inserted meta doc");

    Ok(())
}
