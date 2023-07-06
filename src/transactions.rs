use ethers::{
    providers::{Http, Middleware, Provider},
    types::{H256, Address},
};
use ethers::abi::{Abi, Function, Token};


#[tokio::main]
pub async fn get_transaction_data(abi: Abi, function_name: String, transaction_hash: String) {
    let abi_json = r#"[{
        "inputs": [
          {
            "components": [
              { "internalType": "uint256", "name": "profileId", "type": "uint256" },
              { "internalType": "string", "name": "contentURI", "type": "string" },
              {
                "internalType": "address",
                "name": "collectModule",
                "type": "address"
              },
              {
                "internalType": "bytes",
                "name": "collectModuleInitData",
                "type": "bytes"
              },
              {
                "internalType": "address",
                "name": "referenceModule",
                "type": "address"
              },
              {
                "internalType": "bytes",
                "name": "referenceModuleInitData",
                "type": "bytes"
              }
            ],
            "internalType": "struct DataTypes.PostData",
            "name": "vars",
            "type": "tuple"
          }
        ],
        "name": "post",
        "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
        "stateMutability": "nonpayable",
        "type": "function"
      }]"#;

    let abi: Abi = serde_json::from_str(abi_json).expect("Failed to parse abi");
    let function: &Function = abi.function("post").expect("Function is not found in ABI");

    println!("Running the transactions rust file {:?}", function.inputs);
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

    let input_data: String = transaction.unwrap().input.to_string();
    let input_data = &input_data[10..]; // removing the transaction hash

    let input_bytes = hex::decode(input_data).expect("Failed to decode input bytes");
    let decoded_inputs = function.decode_input(&input_bytes).expect("failed to decode inputs");
    println!("The decoded inputs are {:?}", decoded_inputs);

      
    let vec_tokens = match &decoded_inputs[0] {
      Token::Tuple(tokens) => tokens.to_owned(),
      _ => panic!("Unexpected token type"),
    };
  
    println!("The vec tokens are {:?}", vec_tokens);

    let param1: u64 = match &vec_tokens[0] {
        Token::Uint(value) => value.to_owned().as_u64(), _ => panic!("unexpected token type"),
    };

    println!("The param 1 is {:?}", param1);

    let param2: String = match &vec_tokens[1] {
      Token::String(value) => value.to_owned().to_string(), _ => panic!("unexpected token type"),
    };

    println!("The param 2 is {:?}", param2);

    let param3: Address = match &vec_tokens[2] {
      Token::Address(value) => value.to_owned(), _ => panic!("unexpected token type"),
    };

    println!("The param 3 is {:?}", param3);
    
}
