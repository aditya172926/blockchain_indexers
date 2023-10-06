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
    let db: mongodb::Database = client.database("macha_sdk");
    let collection: mongodb::Collection<Document> = db.collection::<Document>("metas_schemas");

    let meta_schema_pipeline = vec![
        doc! {"$match": {
            "slug": meta_slug
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
