use crate::structs::contracts::{ContractAbi, ContractMetaData};
use crate::structs::index::{MethodParam, MethodParamDataType};
use crate::structs::transactions::{TransactionIndexed, TransactionMethod};
use chrono::Utc;
use ethers::abi::{Abi, Function, Token};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, TransactionReceipt, TxHash},
};
use log::{debug, error, info, warn};
use mongodb::bson::{to_bson, Document};

pub async fn utils_transaction_indexed(
    decoded_txn_data: &(
        TransactionMethod,                 // transaction hash
        ethers::types::TransactionReceipt, // transaction receipt
    ),
    contract_metadata: &ContractMetaData,
) -> TransactionIndexed {
    info!("\ninside utils_trnasaction_indexed\n");
    let block_number_option = decoded_txn_data.1.block_number;
    let block_number = match block_number_option {
        Some(object) => object.as_u64(),
        None => 0,
    };
    let now = Utc::now();
    let ts: String = now.timestamp().to_string();

    let transaction_struct: crate::structs::transactions::Transaction =
        crate::structs::transactions::Transaction {
            block_hash: decoded_txn_data.1.block_hash,
            block_number: block_number,
            contract_address: contract_metadata.contract_address.clone().to_owned(),
            chain_id: contract_metadata.chain_id,
            gas_used: decoded_txn_data.1.gas_used,
            gas_price: decoded_txn_data.1.effective_gas_price,
            from: decoded_txn_data.1.from,
            to: decoded_txn_data.1.to,
            txn_hash: decoded_txn_data.1.transaction_hash,
        };

    let transaction_indexed: TransactionIndexed = TransactionIndexed {
        timestamp: ts,
        transaction: transaction_struct,
        method: decoded_txn_data.0.clone(),
    };
    info!("\ntransaction Indexed = {:?} \n", transaction_indexed);
    transaction_indexed
}

pub async fn utils_transaction_decode<'a>(
    abi: &ContractAbi,
    transaction_hash: TxHash,
    network_rpc_url: &str,
) -> (TransactionMethod, TransactionReceipt) {
    let provider =
        Provider::<Http>::try_from(network_rpc_url).expect("Failed to connect with a Provider");

    // getting the transaction details
    let transaction: Option<ethers::types::Transaction> = provider
        .get_transaction(transaction_hash)
        .await
        .expect("Failed to get the transaction");
    debug!("\n\ntransaction {:?}\n\n", transaction);
    let transaction_receipt_result = provider.get_transaction_receipt(transaction_hash).await;

    let transaction_receipt = match transaction_receipt_result {
        Ok(object) => match object {
            Some(txn_receipt) => txn_receipt,
            None => TransactionReceipt::default(),
        },
        Err(err) => {
            error!("Error in fetching transaction receipt {:?}", err);
            TransactionReceipt::default()
        }
    };

    let transaction_method: TransactionMethod = utils_transaction_method(abi, transaction).await;
    return (transaction_method, transaction_receipt);
}

async fn utils_transaction_method<'a>(
    contract_abi: &ContractAbi,
    transaction: Option<Transaction>,
) -> TransactionMethod {
    let input_data: String = transaction.unwrap().input.to_string();
    let method_id: &str = &input_data[2..10];
    let input_data = &input_data[10..]; // extracting the transaction hash

    if let Some(method) = contract_abi
        .stat
        .functions()
        .into_iter()
        .find(|&f| ethers::utils::hex::encode(f.short_signature()) == method_id)
    {
        info!("Method Name: {}", method.name);
        let param_result: (Vec<MethodParam>, String) =
            utils_transaction_method_params(contract_abi, &method.name, input_data).await;
        let result: TransactionMethod = TransactionMethod {
            name: param_result.1,
            id: method_id.to_string(),
            params: param_result.0,
        };
        result
    } else {
        warn!("Method not found");
        return TransactionMethod {
            params: Vec::new(),
            name: "".to_string(),
            id: "".to_string(),
        };
    }
}

pub async fn utils_transaction_method_params<'a>(
    contract_abi: &ContractAbi,
    method_name: &str,
    input_data: &str,
) -> (Vec<MethodParam>, String) {
    let function: &Function = contract_abi
        .stat
        .function(&method_name)
        .expect("Function is not found in ABI");

    let mut method_params: Vec<MethodParam> = Vec::new();

    let input_bytes: Vec<u8> = hex::decode(input_data).expect("Failed to decode input bytes");
    let decoded_inputs: Vec<Token> = function
        .decode_input(&input_bytes)
        .expect("failed to decode inputs");

    for (index, input) in function.inputs.iter().enumerate() {
        info!(
            "The method_param before formatting ************************ {:?}\n",
            input
        );

        // let mut ind = 0;
        // let name: Result<&Document, mongodb::bson::document::ValueAccessError> =
        //     methods.get_document(method_name);

        // let token_length = match cloned_token.clone().into_tuple() {
        //     Some(i) => i.len(),
        //     None => {
        //         println!("NO COPY TOKEN");
        //         0
        //     }
        // };

        // println!("Total Length:{}", token_length);

        // complex DT
        // let mut input_hashmap: Vec<MethodParam> = Vec::new();
        // if token_length > 0 {
        //     match name {
        //         Ok(i) => {
        //             // let mut input_params: HashMap<String, String> = HashMap::new();
        //             let mut input_params: Vec<MethodParam> = Vec::new();
        //             while ind < token_length - 1 {
        //                 let final_tuple: Option<Vec<Token>> = cloned_token.clone().into_tuple();
        //                 let test = match final_tuple {
        //                     Some(data) => {
        //                         println!("=====DATA====={:?}", data.get(ind));
        //                         let value = data.get(ind).unwrap().to_owned();
        //                         let mut key = i.get_array("params").unwrap()[ind].to_string();
        //                         let input_key = key[1..key.len() - 1].to_string();
        //                         let input_struct: MethodParam = MethodParam {
        //                             name: input_key,
        //                             kind: "".to_string(),
        //                             internal_type: input.internal_type.clone(),
        //                             data_type: MethodParamDataType::StringValue,
        //                             value: ToString::to_string(&value),
        //                         };
        //                         method_params.push(input_struct);
        //                         // input_params.insert(key, value.to_string());
        //                     }
        //                     None => {
        //                         continue;
        //                     }
        //                 };
        //                 ind += 1;
        //             }
        //         }
        //         Err(e) => {
        //             println!("Error {:?}", e);
        //         }
        //     };
        // }

        // println!("INPUT PARAMS=========================={:?}", method_params);

        // let mut method_param: MethodParam;
        // if token_length == 0 {
        //     method_param = MethodParam {
        //         name: String::from(&input.name),
        //         kind: input.kind.to_string(),
        //         internal_type: input.internal_type.clone(),
        //         data_type: crate::structs::index::MethodParamDataType::StringValue,
        //         value: ToString::to_string(&cloned_token),
        //     };
        //     method_params.push(method_param);
        // }
    }
    debug!(
        "The method params are@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {:?}",
        method_params
    );
    return (method_params, method_name.to_string());
}

// Implement a trait for Token
pub trait TokenToString {
    fn to_string(&self) -> String;
}

impl TokenToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Address(addr) => addr.to_string(),
            Token::FixedBytes(bytes) => format!("{:?}", bytes),
            Token::Bytes(bytes) => format!("{:?}", bytes),
            Token::Int(int) => int.to_string(),
            Token::Uint(uint) => uint.to_string(),
            Token::Bool(boolean) => boolean.to_string(),
            Token::String(string) => string.clone(),
            Token::FixedArray(tokens) => {
                let elements: Vec<String> =
                    tokens.iter().map(|t| ToString::to_string(&t)).collect();
                format!("[{}]", elements.join(", "))
            }
            Token::Array(tokens) => {
                let elements: Vec<String> =
                    tokens.iter().map(|t| TokenToString::to_string(t)).collect();
                format!("[{}]", elements.join(", "))
            }
            Token::Tuple(tokens) => {
                let elements: Vec<String> =
                    tokens.iter().map(|t| ToString::to_string(&t)).collect();
                format!("({})", elements.join(", "))
            }
        }
    }
}
