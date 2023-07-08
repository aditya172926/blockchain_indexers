use ethcontract::H256;
use ethers::abi::{Abi, Function, Token};
use ethers::types::H160;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, Transaction, TxHash},
};

pub async fn get_transaction_data(abi: &str, transaction_hash: TxHash) -> (Vec<Token>, String) {
    println!("The transaction hash is {:?}", transaction_hash);
    let provider = Provider::<Http>::try_from("https://lingering-delicate-choice.discover.quiknode.pro/68f9e3726efe97ee2b6a7c8417f6f5d12ab713c6/")
        .expect("Failed to connect with a Provider");

    println!("The transaction hash is {:?}", transaction_hash);

    // getting the transaction details
    let transaction: Option<ethers::types::Transaction> = provider
        .get_transaction(transaction_hash)
        .await
        .expect("Failed to get the transaction");

    let transaction_receipt = provider
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Couldn't get the transaction receipt");

    // println!("The transaction receipt is {:?}", transaction_receipt);
    let decoded_transaction_data: (Vec<Token>, String) = get_transaction_inputs(abi, transaction).await;
    return decoded_transaction_data;
}

async fn get_transaction_inputs(abi: &str, transaction: Option<Transaction>) -> (Vec<ethers::abi::Token>, String) {
    let contract_abi: Abi = serde_json::from_str(&abi).expect("Failed to parse abi");
    let input_data: String = transaction.unwrap().input.to_string();
    let function_id: &str = &input_data[2..10];
    println!("The function id raw is {:?}", function_id);
    let input_data = &input_data[10..]; // removing the transaction hash

    let mut function_name: &str = "";
    if let Some(method) = contract_abi
        .functions()
        .into_iter()
        .find(|&f| ethers::utils::hex::encode(f.short_signature()) == function_id)
    {
        println!("Method Name: {}", method.name);
        function_name = &method.name;
    } else {
        println!("Method not found");
    }

    if (function_name != "") {
        let function: &Function = contract_abi
            .function(&function_name)
            .expect("Function is not found in ABI");

        println!("Running the transactions rust file {:?}", function.inputs);

        let input_bytes = hex::decode(input_data).expect("Failed to decode input bytes");
        let decoded_inputs: Vec<Token> = function
            .decode_input(&input_bytes)
            .expect("failed to decode inputs");
        println!("The decoded inputs are {:?}", decoded_inputs);

        for decoded_input in &decoded_inputs {
            println!("The decoded input is--------- {:?}", decoded_input);
        }

        return (decoded_inputs, function_name.to_string());
    } else {
        println!("Couldn't find the function name");
        return (Vec::new(), "".to_string());
    }
}
