use ethers::abi::Token;

pub fn check_transaction_data (decoded_transaction_data: (Vec<Token>, String)) {
    let decoded_inputs: Vec<Token> = decoded_transaction_data.0;
    let transaction_function_name: String = decoded_transaction_data.1;

    println!("The fn. name in middleware is {}", transaction_function_name);


    // if checks pas add to db
}