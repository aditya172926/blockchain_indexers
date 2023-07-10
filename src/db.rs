use mongodb::{
    bson::{doc, to_bson, Document, Binary, Array, Bson},
    options::{ClientOptions, ServerApi},
    Client,
};
// use ethers::{abi::Event};
use ethcontract::{RawLog, H256};
use ethers::abi::{ParamType};
use serde_json::{json, Value};
use serde::{Serialize, Serializer};
use crate::structs::MethodParam;

#[derive(Serialize)]
struct BytesWrapper<'a> {
    #[serde(serialize_with = "serialize_bytes")]
    bytes: &'a[u8]
}

fn serialize_bytes<S>(bytes: &&[u8], serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    // let bson_binary = Binary {
    //     subtype: mongodb::bson::spec::BinarySubtype::Generic,
    //     bytes: bytes.to_owned().to_vec()
    // };
    // let mut array: Vec<mongodb::bson::Bson> = Array::new();
    // array.push(bson_binary);
    // array.serialize(serializer);

    let bson_array = bytes.iter().map(|&byte| Bson::Int32(byte as i32)).collect::<Vec<Bson>>();
    serializer.serialize_some(&bson_array)
}

pub async fn save_to_db(event: RawLog) -> Result<(), Box<dyn std::error::Error>> {

    let mut json_object = json!({});
    json_object["topics"] = serde_json::to_value(&event.topics).unwrap();
    let data_wrapper = BytesWrapper{bytes: &event.data};
    let bson_data: Bson = to_bson(&data_wrapper).unwrap();
    println!("The bson data is {:?}", bson_data);
    json_object["data"] = Value::from(event.data);
    // let event_bson: mongodb::bson::Bson = to_bson(&json_object).unwrap();

    // println!("The bson object is {:?}", event_bson);

    // json_object["data"] = &event.data;
    let client_options = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("macha_sdk");
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


pub async fn save_txn_to_db(txn: Vec<MethodParam<'_>>) -> Result<(), Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("macha_sdk");
    let collection = db.collection::<Document>("transactions");

    let event_bson: mongodb::bson::Bson = to_bson(&txn).unwrap();

    let event_document = doc! {
        "transaction": event_bson,
        "timestamp": "Testingt time stamp",
    };

    println!("The event document is {:?}", event_document);

    collection.insert_one(event_document, None).await?;

    println!("Event document inserted successfully!");

    Ok(())
}
