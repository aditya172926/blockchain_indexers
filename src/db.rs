use mongodb::{bson::{doc, Document}, Client, options::ClientOptions};
use ethers::contract::{Event};


pub fn save_to_db(event: Event) -> Result<(), Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("eventsdb");
    let collection = db.collection::<Document>("events");

    let event_document = doc! {
        "event": event,
        "timestamp": chrono::Utc::now(),
    };

    collection.insert_one(event_document, None).await?;

    println!("Event document inserted successfully!");

    Ok(())
}
