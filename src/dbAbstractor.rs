use crate::structs::Meta;
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    error::Error,
    options::ClientOptions,
    Client, Cursor,
};


pub async fn get_meta_schema(meta_slug: &str) -> Option<Document> {
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


pub async fn update_block_number(
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

pub async fn upload_meta_to_db(
    meta: Meta,
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
