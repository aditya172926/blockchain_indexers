use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    options::ClientOptions,
    Client
};
use crate::structs::{MethodParam, TransactionData, ContractData};
use ethcontract::RawLog;
use ethers::types::TransactionReceipt;
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
    // let bson_binary = Binary {
    //     subtype: mongodb::bson::spec::BinarySubtype::Generic,
    //     bytes: bytes.to_owned().to_vec()
    // };
    // let mut array: Vec<mongodb::bson::Bson> = Array::new();
    // array.push(bson_binary);
    // array.serialize(serializer);

    let bson_array = bytes
        .iter()
        .map(|&byte| Bson::Int32(byte as i32))
        .collect::<Vec<Bson>>();
    serializer.serialize_some(&bson_array)
}

// async fn get_db_client(collection_name: String) -> Result<mongodb::Collection<Document>, Box<dyn std::error::Error>> {
//     let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
//     let client: Client = Client::with_options(client_options)?;
//     let db: mongodb::Database = client.database("macha_sdk");
//     let collection: mongodb::Collection<Document> = db.collection::<Document>("transactions");
//     Ok(());
// }

pub async fn save_to_db(event: RawLog) -> Result<(), Box<dyn std::error::Error>> {
    let mut json_object = json!({});
    json_object["topics"] = serde_json::to_value(&event.topics).unwrap();
    let data_wrapper = BytesWrapper { bytes: &event.data };
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

pub async fn save_txn_to_db(
    txn: Vec<MethodParam<'_>>,
    method_name: String,
    method_id: String,
    transaction_receipt: TransactionReceipt,
    contract_address: String,
    contract_slug: String
) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("macha_sdk");
    let collection: mongodb::Collection<Document> = db.collection::<Document>("transactions");

    let transaction_struct: TransactionData = TransactionData {
        block_hash: transaction_receipt.block_hash,
        block_number: transaction_receipt.block_number,
        contract_slug: contract_slug,
        contract_address: contract_address,
        gas_used: transaction_receipt.gas_used,
        gas_price: transaction_receipt.effective_gas_price,
        from: transaction_receipt.from,
        to: transaction_receipt.to,
        txn_hash: transaction_receipt.transaction_hash,
        method_name: method_name,
        method_id: method_id,
        method_params: txn
    };

    // let event_bson: mongodb::bson::Bson = to_bson(&txn).unwrap();
    let transaction_bson_receipt: mongodb::bson::Bson = to_bson(&transaction_struct).unwrap();
    let event_document = doc! {
        "transaction": transaction_bson_receipt,
        "timestamp": "Testingt time stamp",
    };
    println!("The event document is {:?}", event_document);
    collection.insert_one(event_document, None).await?;
    println!("Event document inserted successfully!");
    Ok(())
}

pub async fn save_contract_to_db (contract_data: ContractData) -> Result<(), Box<dyn std::error::Error>> {
    let client_options: ClientOptions = ClientOptions::parse("mongodb+srv://metaworkdao:c106%40bh1@cluster0.h2imk.mongodb.net/metawork?retryWrites=true&w=majority").await?;
    let client: Client = Client::with_options(client_options)?;
    let db: mongodb::Database = client.database("macha_sdk");
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