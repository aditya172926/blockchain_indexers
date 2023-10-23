use ethcontract::Instance;
use ethers::core::types::Chain;
use ethers::etherscan::Client;
use ethers::prelude::*;
use ethers::types::H256;
use futures::StreamExt;
use log::{debug, error, info, warn};
use mongodb::bson::Document;
use std::collections::HashSet;
use std::str::FromStr;
use web3::transports::Http;

use ethers::{prelude::account::Sort, providers::Provider};

use ethers::etherscan::account::TxListParams;

use crate::handlers::lens_post::handler_lens_post;
use crate::handlers::lens_profile_polygon::handler_lens_profile;
use crate::handlers::poap_ethereum::handler_poap_ethereum;
use crate::structs::contracts::{ContractAbi, ContractMetaData};
use crate::structs::extract::Config;
use crate::structs::meta::{self, MetaIndexed, MetaSubStruct};
use crate::structs::networks::NetworkStruct;
use crate::structs::transactions::{TransactionIndexed, TransactionMethod};
use crate::utils::index::utils_interesting_method;
use crate::utils::meta::utils_meta_indexed;
use crate::utils::transactions::utils_transaction_indexed;
use crate::{structs, utils};
use std::process::exit;
use tokio::time::{sleep, Duration};

async fn load_txns(
    config: Config,
    contract_abi: &ContractAbi,
    transaction_hash: H256,
    network_metadata: NetworkStruct,
    contract_metadata: ContractMetaData,
) {
    let mut decoded_txn_data: (
        TransactionMethod,
        ethers::types::TransactionReceipt, // transaction receipt
    ) = utils::transactions::utils_transaction_decode(
        contract_abi,
        transaction_hash,
        &network_metadata.network_rpc_url,
    )
    .await;

    if decoded_txn_data.0.name != "".to_string()
        && utils_interesting_method(
            &contract_metadata.method_of_interest,
            &decoded_txn_data.0.name,
        )
    {
        let transaction_indexed: TransactionIndexed =
            utils_transaction_indexed(&decoded_txn_data, &contract_metadata).await;

        let meta_indexed: MetaIndexed = utils_meta_indexed(&config, transaction_indexed).await;
        info!("meta_indexed -> {:?}", meta_indexed);
        // abstractor::create_meta(&contract_slug,transaction_indexed).await;

        // let _ = db::db_transaction_store(
        //     decoded_txn_data.0, //method_params
        //     decoded_txn_data.1, // function name
        //     decoded_txn_data.2, // function id
        //     decoded_txn_data.3, // transaction receipt
        //     contract_address.clone(),
        //     String::from(&contract_slug),
        //     &chain_id,
        // )
        // .await;
    }
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
}

pub async fn get_txns(
    config: &Config,
    contract_abi: &ContractAbi,
    contract_instance: &Instance<Http>,
    contract_metadata: ContractMetaData,
    network_metadata: &NetworkStruct,
) {
    let event_stream = contract_instance
        .all_events()
        .from_block(ethcontract::BlockNumber::from(
            network_metadata.start_block_number,
        ))
        .stream();
    let mut event_stream = Box::pin(event_stream);
    let mut prev_txn_hash: H256 =
        H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
            .unwrap();
    loop {
        match event_stream.next().await {
            Some(Ok(log)) => {
                let txn_hash = log.meta.as_ref().unwrap().transaction_hash.to_fixed_bytes();
                let transaction_hash: H256 = ethers::core::types::TxHash::from(txn_hash);
                info!("\nTransactionHash -> {:?}\n", transaction_hash);

                if transaction_hash != prev_txn_hash {
                    load_txns(
                        config.to_owned(),
                        contract_abi,
                        transaction_hash,
                        network_metadata.clone(),
                        contract_metadata.clone(),
                    )
                    .await;
                    prev_txn_hash = transaction_hash;
                }
            }
            Some(Err(e)) => {
                error!("Error: {:?}", e);
                continue;
            }
            None => {
                warn!("Stream ended, reconnecting...");
                sleep(Duration::from_secs(2)).await;

                event_stream = Box::pin(
                    contract_instance
                        .all_events()
                        .from_block(ethcontract::BlockNumber::from(
                            network_metadata.start_block_number,
                        ))
                        .stream(),
                );
            }
        }
    }
}

pub async fn get_history(
    config: &Config,
    contract_metadata: &ContractMetaData,
    network_metadata: &NetworkStruct,
    contract_abi: &ContractAbi,
) -> eyre::Result<()> {
    let _provider = Provider::try_from(network_metadata.network_rpc_url.clone())?;

    let chain_type = match network_metadata.network_id {
        1 => Chain::Mainnet,
        137 => Chain::Polygon,
        u64::MIN..=0_u64 | 2_u64..=136_u64 | 138_u64..=u64::MAX => Chain::Mainnet,
    };
    // etherscan client builder
    let client = Client::builder()
        .with_api_key(network_metadata.network_api_key.clone())
        .chain(chain_type)
        .unwrap()
        .build()
        .unwrap();

    let params = TxListParams {
        start_block: contract_metadata.start_block,
        end_block: contract_metadata.end_block,
        page: 0,
        offset: 0,
        sort: Sort::Asc,
    };

    let mut prev_txn_hash: H256 =
        H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
            .unwrap();

    //Fetching all transactions
    info!("\n {} \n", contract_metadata.contract_address_historical);

    let txns = client
        .get_transactions(
            &contract_metadata
                .contract_address_historical
                .parse()
                .unwrap(),
            Some(params),
        )
        .await
        .unwrap();

    //Creating loop to iterate over all transactions
    for txn in txns {
        let txn_hash = txn.hash.value().unwrap().to_fixed_bytes();
        let transaction_hash: H256 = ethers::core::types::TxHash::from(txn_hash);
        info!("\ntransaction hash {:?}\n", transaction_hash);

        if transaction_hash != prev_txn_hash {
            load_txns(
                config.to_owned(),
                contract_abi,
                transaction_hash,
                network_metadata.clone(),
                contract_metadata.clone(),
            )
            .await;
            prev_txn_hash = transaction_hash;
        }
    }

    Ok(())
}
