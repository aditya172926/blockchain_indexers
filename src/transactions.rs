use std::any::Any;
use std::clone;
use std::collections::HashMap;

use crate::structs::{MethodParam, MethodParamvalue};
use ethers::abi::{Abi, Function, Token};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, TransactionReceipt, TxHash},
};
use mongodb::bson::Document;

pub async fn get_transaction_data<'a>(
    abi: &str,
    transaction_hash: TxHash,
    network_rpc_url: &str,
    methods: &Document,
) -> (Vec<MethodParam<'a>>, String, String, TransactionReceipt) {
    println!("The transaction hash is {:?}", transaction_hash);

    let provider =
        Provider::<Http>::try_from(network_rpc_url).expect("Failed to connect with a Provider");

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
        get_transaction_inputs(contract_abi, transaction, methods).await;

    return (
        decoded_transaction_data.0, // method_params
        decoded_transaction_data.1, // function name
        decoded_transaction_data.2, // function id
        transaction_receipt,
    );
}

async fn get_transaction_inputs<'a>(
    contract_abi: &'static Abi,
    transaction: Option<Transaction>,
    methods: &Document,
) -> (Vec<MethodParam<'a>>, String, String) {
    let input_data: String = transaction.unwrap().input.to_string();
    let function_id: &str = &input_data[2..10];
    let input_data = &input_data[10..]; // extracting the transaction hash

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

        let mut method_params: Vec<MethodParam> = Vec::new();

        let input_bytes: Vec<u8> = hex::decode(input_data).expect("Failed to decode input bytes");
        let decoded_inputs: Vec<Token> = function
            .decode_input(&input_bytes)
            .expect("failed to decode inputs");

        for (index, input) in function.inputs.iter().enumerate() {
            let cloned_token: Token = decoded_inputs[index].clone();
            println!("The cloned token is {:?}", cloned_token);
            println!(
                "The method_param before formatting ************************ {:?}",
                input
            );

            let mut ind = 0;
            let mut token_length = 0;
            let name: Result<&Document, mongodb::bson::document::ValueAccessError> = methods.get_document(function_name);

            match cloned_token.clone().into_tuple() {
                Some(i) => {
                    token_length = i.len();
                }
                None => {
                    println!("NO COPY TOKEN");
                }
            }

            println!("{}", token_length);
            let input_hashmap: HashMap<String, String> = match name {
                Ok(i) => {
                    let mut input_params: HashMap<String, String> = HashMap::new();
                    while ind < token_length - 1 {
                        let final_tuple: Option<Vec<Token>> = cloned_token.clone().into_tuple();
                        match final_tuple {
                            Some(data) => {
                                println!("=====DATA====={:?}", data.get(ind));
                                let value = data.get(ind).unwrap().to_owned();
                                let mut key = i.get_array("params").unwrap()[ind].to_string();
                                key = key[1..key.len()-1].to_string();
                                input_params.insert(key, value.to_string());
                            }
                            None => {
                                continue;
                            }
                        }

                        // println!("============KEY:VALUE===={:?}",value);
                        ind += 1;
                    }
                    println!("The input params are {:?}", input_params);
                    input_params
                }
                Err(e) => {
                    println!("Error {:?}", e);
                    let input_params: HashMap<String, String> = HashMap::new();
                    input_params
                }
            };
            println!("INPUT PARAMS=========================={:?}", input_hashmap);

            let method_param: MethodParam;
            if input_hashmap.is_empty() {
                method_param = MethodParam {
                    name: &input.name,
                    kind: input.kind.to_string(),
                    internal_type: &input.internal_type,
                    value: MethodParamvalue::StringValue(cloned_token.to_string()),
                };
            } else {
                method_param = MethodParam {
                    name: &input.name,
                    kind: input.kind.to_string(),
                    internal_type: &input.internal_type,
                    value: MethodParamvalue::ComplexData(input_hashmap),
                };
            }
            
            println!(
                "The generated method param object -------> {:?}",
                method_param
            );
            method_params.push(method_param);
        }
        println!("The method params are@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {:?}", method_params);
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
