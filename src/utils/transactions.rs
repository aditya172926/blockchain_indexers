use std::process::exit;

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
    // exit(1);
    let transaction_receipt_result = provider.get_transaction_receipt(transaction_hash).await;
    println!("\n\ntransaction {:?}\n\n", transaction_receipt_result);
    
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
    info!("Metod id found in txn : {} ", method_id);
    if let Some(method) = contract_abi
        .stat
        .functions()
        .into_iter()
        .find(|&f| ethers::utils::hex::encode(f.short_signature()) == method_id)
    {
        info!("Method Name found in abi: {}", method.name);
        let method_name = &method.name;
        let param_result: Vec<Token> =
            utils_transaction_method_params(contract_abi, method_name, input_data).await;
        let result: TransactionMethod = TransactionMethod {
            name: method_name.to_string(),
            id: method_id.to_string(),
            params: param_result,
        };
        result
    } else {
        warn!("Method not found in abi ");
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
) -> Vec<Token> {
    let function: &Function = contract_abi
        .stat
        .function(&method_name)
        .expect("Function is not found in ABI");

    let input_bytes: Vec<u8> = hex::decode(input_data).expect("Failed to decode input bytes");
    let decoded_inputs: Vec<Token> = function
        .decode_input(&input_bytes)
        .expect("failed to decode inputs");

    decoded_inputs
}
