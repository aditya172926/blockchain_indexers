pub async fn get_events(
    contract_instance: Instance<Http>,
    block_number: i64,
) -> Result<(), Box<dyn Error>> {
    // Subscribe to all events
    let mut event_streams = contract_instance
        .all_events()
        .from_block(ethcontract::BlockNumber::from(block_number))
        .stream()
        .boxed();

    println!("waiting for events.......");
    loop {
        join! {
            async {
                let log = event_streams.next().await.expect("No events").expect("Error querying event").added();
                let unwrapped_log = log.unwrap();
                // let _ = db::db_event_store(unwrapped_log).await;
            },
        };
    }
}