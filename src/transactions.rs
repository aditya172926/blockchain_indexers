use crate::structs::MethodParam;
use ethers::abi::{Abi, Function, Token};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, TransactionReceipt, TxHash},
};

pub async fn get_transaction_data<'a>(
    abi: &str,
    transaction_hash: TxHash,
    network_rpc_url: &str
) -> (Vec<MethodParam<'a>>, String, String, TransactionReceipt) {
    println!("The transaction hash is {:?}", transaction_hash);

    let provider = Provider::<Http>::try_from(network_rpc_url)
        .expect("Failed to connect with a Provider");

    // getting the transaction details
    let transaction: Option<ethers::types::Transaction> = provider
        .get_transaction(transaction_hash)
        .await
        .expect("Failed to get the transaction");
    let transaction_receipt = provider
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Couldn't get the transaction receipt");
    let transaction_receipt: TransactionReceipt = transaction_receipt.unwrap();
    let contract_abi: &'static Abi = Box::leak(Box::new(
        serde_json::from_str(&abi).expect("Failed to parse abi"),
    ));
    let decoded_transaction_data: (Vec<MethodParam>, String, String) =
        get_transaction_inputs(contract_abi, transaction).await;

    return (
        decoded_transaction_data.0,
        decoded_transaction_data.1,
        decoded_transaction_data.2,
        transaction_receipt,
    );
}

async fn get_transaction_inputs(
    contract_abi: &'static Abi,
    transaction: Option<Transaction>,
) -> (Vec<MethodParam>, String, String) {
    let input_data: String = transaction.unwrap().input.to_string();
    let function_id: &str = &input_data[2..10];
    // println!("The function id raw is {:?}", function_id);
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

    if function_name != "" {
        let function: &Function = contract_abi
            .function(&function_name)
            .expect("Function is not found in ABI");

        // println!("Running the transactions rust file {:?}", function.inputs);

        let mut method_params: Vec<MethodParam> = Vec::new();


        let input_bytes: Vec<u8> = hex::decode(input_data).expect("Failed to decode input bytes");
        let decoded_inputs: Vec<Token> = function
            .decode_input(&input_bytes)
            .expect("failed to decode inputs");

        for (index, input) in function.inputs.iter().enumerate() {
            let cloned_token = decoded_inputs[index].clone();
            let method_param: MethodParam = MethodParam {
                name: &input.name,
                kind: input.kind.to_string(),
                internal_type: &input.internal_type,
                value: cloned_token,
            };
            method_params.push(method_param);
        }
        println!("The method params are {:?}", method_params);
        return (
            method_params,
            function_name.to_string(),
            function_id.to_string(),
        );
    } else {
        println!("Couldn't find the function name");
        return (Vec::new(), "".to_string(), "".to_string());
    }
}
