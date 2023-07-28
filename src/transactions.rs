use std::any::Any;
use std::clone;
use std::collections::HashMap;
use std::str::FromStr;

use crate::structs::{MethodParam, MethodParamDataType, MethodParamValue};
use ethcontract::H256;
use ethers::abi::{Abi, Function, Token};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, TransactionReceipt, TxHash},
};
use mongodb::bson::Document;
use serde_json::from_str;

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
    let transaction_receipt: Option<
        ethers::core::types::transaction::response::TransactionReceipt,
    > = provider
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Couldn't get the transaction receipt");

    let transaction_receipt_formatted:ethers::core::types::transaction::response::TransactionReceipt;
    match transaction_receipt {
        (txn) => {
            match txn {
                Some(object) => {
                    transaction_receipt_formatted = object;
                },
                None => {
                    transaction_receipt_formatted = TransactionReceipt::default();
                }
            }
            // transaction_receipt_formatted = txn;
        }
        _ => {
            transaction_receipt_formatted = TransactionReceipt::default();
        }
    }

    let contract_abi: &'static Abi = Box::leak(Box::new(
        serde_json::from_str(&abi).expect("Failed to parse abi"),
    ));
    let decoded_transaction_data: (Vec<MethodParam>, String, String) =
        get_transaction_method(contract_abi, transaction, methods).await;

    return (
        decoded_transaction_data.0, // method_params
        decoded_transaction_data.1, // method name
        decoded_transaction_data.2, // method id
        transaction_receipt_formatted,
    );
}

async fn get_transaction_method<'a>(
    contract_abi: &'static Abi,
    transaction: Option<Transaction>,
    methods: &Document,
) -> (Vec<MethodParam<'a>>, String, String) {
    let input_data: String = transaction.unwrap().input.to_string();
    let method_id: &str = &input_data[2..10];
    let input_data = &input_data[10..]; // extracting the transaction hash

    let mut method_name: &str = "";
    if let Some(method) = contract_abi
        .functions()
        .into_iter()
        .find(|&f| ethers::utils::hex::encode(f.short_signature()) == method_id)
    {
        println!("Method Name: {}", method.name);
        method_name = &method.name;
    } else {
        println!("Method not found");
    }

    if method_name != "" {
        let param_result =
            get_transaction_method_params(contract_abi, method_name, input_data, methods).await;
        return (param_result.0, param_result.1, method_id.to_string());
    } else {
        println!("Couldn't find the function name");
        return (Vec::new(), "".to_string(), "".to_string());
    }
}

pub async fn get_transaction_method_params<'a>(
    contract_abi: &'static Abi,
    method_name: &str,
    input_data: &str,
    methods: &Document,
) -> (Vec<MethodParam<'a>>, String) {
    let function: &Function = contract_abi
        .function(&method_name)
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
        let name: Result<&Document, mongodb::bson::document::ValueAccessError> =
            methods.get_document(method_name);

        let token_length = match cloned_token.clone().into_tuple() {
            Some(i) => i.len(),
            None => {
                println!("NO COPY TOKEN");
                0
            }
        };

        println!("{}", token_length);

        // complex DT
        // let mut input_hashmap: Vec<MethodParam> = Vec::new();
        if token_length > 0 {
            match name {
                Ok(i) => {
                    // let mut input_params: HashMap<String, String> = HashMap::new();
                    let mut input_params: Vec<MethodParam> = Vec::new();
                    while ind < token_length - 1 {
                        let final_tuple: Option<Vec<Token>> = cloned_token.clone().into_tuple();
                        let test = match final_tuple {
                            Some(data) => {
                                println!("=====DATA====={:?}", data.get(ind));
                                let value = data.get(ind).unwrap().to_owned();
                                let mut key = i.get_array("params").unwrap()[ind].to_string();
                                let input_key = key[1..key.len() - 1].to_string();
                                let input_struct: MethodParam = MethodParam {
                                    name: input_key,
                                    kind: "".to_string(),
                                    internal_type: &input.internal_type,
                                    data_type: MethodParamDataType::StringValue,
                                    value: value.to_string(),
                                };
                                method_params.push(input_struct);
                                // input_params.insert(key, value.to_string());
                            }
                            None => {
                                continue;
                            }
                        };
                        ind += 1;
                    }
                }
                Err(e) => {
                    println!("Error {:?}", e);
                }
            };
        }

        // println!("INPUT PARAMS=========================={:?}", input_hashmap);

        let mut method_param: MethodParam;
        if token_length == 0 {
            method_param = MethodParam {
                name: String::from(&input.name),
                kind: input.kind.to_string(),
                internal_type: &input.internal_type,
                data_type: crate::structs::MethodParamDataType::StringValue,
                value: cloned_token.to_string(),
            };
            method_params.push(method_param);
        }
    }
    println!(
        "The method params are@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {:?}",
        method_params
    );
    return (method_params, method_name.to_string());
}
