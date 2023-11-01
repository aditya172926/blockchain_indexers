use ethers::types::H160;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::process::exit;

use crate::handlers::ens_ethereum::handler;
use crate::structs::extract::Schema;
use crate::structs::meta::{Meta, MetaIndexed, MetaResult};
use crate::structs::transactions::TransactionIndexed;

pub async fn handler_ens(
    transaction_indexed: &TransactionIndexed,
    schema: &Schema,
) -> Option<MetaResult> {
    match &transaction_indexed.method {
        Some(transaction_indexed_method) => {
            if transaction_indexed_method.name == "register"
                || transaction_indexed_method.name == "registerOnly"
            {
                return handler::handler_txn_register_ens(transaction_indexed, schema).await;
            } else if transaction_indexed_method.name == "renew" {
                return handler::handler_txn_renew_ens(
                    transaction_indexed,
                    transaction_indexed_method,
                    schema,
                )
                .await;
            } else if transaction_indexed_method.name == "reclaim" {
                return handler::handler_txn_reclaim_ens(
                    transaction_indexed,
                    transaction_indexed_method,
                    schema,
                )
                .await;
            } else {
                return None;
            }
        }

        None => {
            let transaction_event = transaction_indexed.event.as_ref().unwrap();
            println!("{}", transaction_event.params[0].to_string());
            if transaction_event.name.to_string() == "NameRegistered" {
                return handler::handler_event_register_ens_by_controller(
                    transaction_indexed,
                    schema,
                )
                .await;
            } else if transaction_event.name.to_string() == "NameRenewed" {
                return handler::handler_event_renew_ens_by_controller(transaction_indexed, schema)
                    .await;
            } else if transaction_event.name.to_string() == "Transfer" {
                return handler::handler_event_transfer_ens(
                    transaction_indexed,
                    transaction_event.clone(),
                    schema,
                )
                .await;
            } else {
                return None;
            }
        }
    }
}
