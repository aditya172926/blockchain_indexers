use ethcontract::{Http, Instance};
use log::{debug, error, info, warn};

use crate::utils::index::utils_url_data;

pub async fn helper_erc721(token_id: u128, contract_instance: Instance<Http>) {
    // let get_token_url = contract_instance.method::<_, String>("tokenURI", token_id);

    // let token_url = match get_token_url {
    //     Ok(method) => {
    //         let token_url_promise = method.call().await;
    //         match token_url_promise {
    //             Ok(result) => result,
    //             Err(e) => {
    //                 error!("Error in contract call -> {:?}\n\n", e);
    //                 String::new()
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         error!(
    //             "Error in get_token_url method from the smart contract {:?}\n\n",
    //             e
    //         );
    //         String::new()
    //     }
    // };

    // if !token_url.is_empty() {
    //     let response = utils_url_data(&token_url).await;
    //     // let response: Result<reqwest::Response, reqwest::Error> = get(&token_url).await;
    //     match response {
    //         Ok(object) => {
    //             let nft_data = object
    //                 .text()
    //                 .await
    //                 .expect("Error in parsing nft data to string\n");
    //             info!("The nft data is {:?}\n\n", nft_data);
    //             let nft_data_content =
    //                 serde_json::from_str(&nft_data).expect("error in reading json format");
    //             println!("\n\n\nNft data content {} \n\n\n", nft_data_content)
    //             // let mut erc721_data_hashmap: HashMap<String, serde_json::Value> = HashMap::new();
    //             // erc721_data_hashmap.insert(String::from(&param.name), nft_data_content);
    //             // metadata_list.insert("erc721_module".to_string(), erc721_data_hashmap);
    //         }
    //         Err(error) => {
    //             error!("Error in fetching nft data from url {:?}\n\n", error);
    //         }
    //     }
    // }
}
