use ethcontract::H256;
use ethers::abi::{Abi, Function, Token, ParamType};
use ethers::types::H160;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, Transaction, TxHash, TransactionReceipt},
};
use mongodb::bson::Array;
use crate::structs::MethodParam;

pub async fn get_transaction_data(abi: &str, transaction_hash: TxHash) -> (Vec<MethodParam<'_>>, String, TransactionReceipt) {
    println!("The transaction hash is {:?}", transaction_hash);
    let provider = Provider::<Http>::try_from("https://lingering-delicate-choice.discover.quiknode.pro/68f9e3726efe97ee2b6a7c8417f6f5d12ab713c6/")
        .expect("Failed to connect with a Provider");

    // println!("The transaction hash is {:?}", transaction_hash);

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
    let contract_abi: &'static Abi = Box::leak(Box::new(serde_json::from_str(&abi).expect("Failed to parse abi")));
    let decoded_transaction_data: (Vec<MethodParam<'static>>, String) = get_transaction_inputs(contract_abi, transaction).await;
    return (decoded_transaction_data.0, decoded_transaction_data.1, transaction_receipt);

}

async fn get_transaction_inputs(contract_abi: &'static Abi, transaction: Option<Transaction>) -> (Vec<MethodParam<'static>>, String) {
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

    if (function_name != "") {
        let function: &Function = contract_abi
            .function(&function_name)
            .expect("Function is not found in ABI");

        println!("Running the transactions rust file {:?}", function.inputs);

        let mut method_params: Vec<MethodParam<'_>> = Vec::new();
;        let mut index: usize = 0;
        for input in &function.inputs {
            println!("******* The input name is {:?} *******", input.name);
            println!("******* The input kind is {:?} *******", input.kind);
            println!("******* The input internal type is {:?} *******", input.internal_type);
        }

        let input_bytes: Vec<u8> = hex::decode(input_data).expect("Failed to decode input bytes");
        let decoded_inputs: Vec<Token> = function
            .decode_input(&input_bytes)
            .expect("failed to decode inputs");
        
        // while &index < &owned_decoded_inputs.len() {
        //     let current_input = &function.inputs[index];
        //     let method_param: MethodParam = MethodParam {
        //         name: &current_input.name,
        //         kind: &current_input.kind,
        //         internal_type: &current_input.internal_type,
        //         value: &owned_decoded_inputs[index]
        //     };
        //     // println!("The Method params are {:?} ", method_param);
        //     method_params.push(method_param);
        //     index += 1;
        // }
        for (index, input) in function.inputs.iter().enumerate() {
            let current_input = &input;
            let cloned_token = decoded_inputs[index].clone();
            let method_param: MethodParam<'static> = MethodParam {
                name: &current_input.name,
                kind: &current_input.kind,
                internal_type: &current_input.internal_type,
                value: cloned_token,
            };
            println!("The Method params are {:?} ", method_param);
            method_params.push(method_param);
        }
        return (method_params, function_name.to_string());
    } else {
        println!("Couldn't find the function name");
        return (Vec::new(), "".to_string());
    }
}
