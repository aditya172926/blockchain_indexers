use ethers::{
    core::types::Bytes,
    providers::{Http, Middleware, Provider},
    types::{TransactionRequest, H256},
};

#[tokio::main]
async fn main() {
    println!("Running the transactions rust file ");
    // connect to a network -> eg Polygon
    let provider = Provider::<Http>::try_from("https://rpc-mainnet.maticvigil.com")
        .expect("Failed to connect with a Provider");

    let transaction_hash: H256 =
        "0x6b69174c0969eda83feb75734fee22722b518aba79be76aaa839ae58fd44d58b"
            .parse()
            .expect("Failed to parse transaction hash");

    // getting the transaction details
    let transaction = provider
        .get_transaction(transaction_hash)
        .await
        .expect("Failed to get the transaction");

    let transaction_receipt = provider
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Couldn't get the transaction receipt");

    // println!("The transaction is {:?}", transaction);
    // println!("------------------------------------------------------");
    // println!("The transaction receipt is {:?} ", transaction_receipt);

    let input_data = transaction.unwrap().input;
    println!("The inputs are {:?}", input_data);
}
