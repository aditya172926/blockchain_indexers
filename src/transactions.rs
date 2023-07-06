use ethers::abi::{Abi, Function, Token};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, H256},
};

#[tokio::main]
pub async fn get_transaction_data(abi: &str, function_name: String, transaction_hash: String) {
    let abi: Abi = serde_json::from_str(abi).expect("Failed to parse abi");
    println!("The decoded abi is {:?}", abi);
    let function: &Function = abi
        .function(&function_name)
        .expect("Function is not found in ABI");

    println!("Running the transactions rust file {:?}", function.inputs);
    // connect to a network -> eg Polygon
    let provider = Provider::<Http>::try_from("https://rpc-mainnet.maticvigil.com")
        .expect("Failed to connect with a Provider");
    println!("GOt the provider {:?}", provider);

    let transaction_hash: H256 = transaction_hash
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

    let input_data: String = transaction.unwrap().input.to_string();
    let input_data = &input_data[10..]; // removing the transaction hash

    let input_bytes = hex::decode(input_data).expect("Failed to decode input bytes");
    let decoded_inputs = function
        .decode_input(&input_bytes)
        .expect("failed to decode inputs");
    println!("The decoded inputs are {:?}", decoded_inputs);

    let vec_tokens = match &decoded_inputs[0] {
        Token::Tuple(tokens) => tokens.to_owned(),
        _ => panic!("Unexpected token type"),
    };

    println!("The vec tokens are {:?}", vec_tokens);

    let param1: u64 = match &vec_tokens[0] {
        Token::Uint(value) => value.to_owned().as_u64(),
        _ => panic!("unexpected token type"),
    };

    println!("The param 1 is {:?}", param1);

    let param2: String = match &vec_tokens[1] {
        Token::String(value) => value.to_owned().to_string(),
        _ => panic!("unexpected token type"),
    };

    println!("The param 2 is {:?}", param2);

    let param3: Address = match &vec_tokens[2] {
        Token::Address(value) => value.to_owned(),
        _ => panic!("unexpected token type"),
    };

    println!("The param 3 is {:?}", param3);
}
