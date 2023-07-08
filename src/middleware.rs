use ethers::abi::Token;

pub fn check_transaction_data (decoded_transaction_data: (Vec<Token>, String), function_of_interest: &str) {
    let decoded_inputs: Vec<Token> = decoded_transaction_data.0;
    let transaction_function_name: String = decoded_transaction_data.1;

    println!("The fn. name in middleware is {}", transaction_function_name);
    if (transaction_function_name == function_of_interest) {
        // if this is the correct function. Push in the db
    } else {
        println!("Either function was not found, or didn't match the function of Interest {}", function_of_interest);
    }
}