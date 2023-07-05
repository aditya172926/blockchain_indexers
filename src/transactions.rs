use ethers::{
    core::types::Bytes,
    providers::{Http, Middleware, Provider},
    types::{TransactionRequest, H256},
};
use ethers::abi::{Abi, Function};
use hex::decode;

#[tokio::main]
pub async fn get_transaction_data() {
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

    // println!("The transaction is {:?}", transaction);
    // println!("------------------------------------------------------");
    // println!("The transaction receipt is {:?} ", transaction_receipt);

    let input_data = transaction.unwrap().input;
    println!("The inputs are {:?}", input_data);

    let sample_input = "0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000001a8dc00000000000000000000000000000000000000000000000000000000000000c000000000000000000000000054325d507ed1b3776c146700eff61a98b45aec76000000000000000000000000000000000000000000000000000000000000012000000000000000000000000017317f96f0c7a845ffe78c60b10ab15789b57aaa0000000000000000000000000000000000000000000000000000000000000220000000000000000000000000000000000000000000000000000000000000003061723a2f2f337550425559524a424863383849376d5f676c6267626777613135694f473251514e4b3663317653702d380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000016345785d8a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d500b1d8e8ef31e21c99d1db9a6444d3adf1270000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000321737a45f7942078ea5ad9d41ec9d163f16813a0000000000000000000000000000000000000000000000000000000000000000";
    // let sample_input_bytes = decode(sample_input).expect("Failed to decode ");

    let input_bytes = hex::decode(sample_input).expect("Failed to decode input bytes");
    println!("Here are the input bytes {:?}", input_bytes);
    let decoded_inputs = function.decode_input(&input_bytes).expect("failed to decode inputs");
    println!("The inputs are {:?}", decoded_inputs);
}
