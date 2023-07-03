use std::fmt::Debug;
use std::fs::File;
use std::net::SocketAddr;
use std::error::Error;
use std::fs;
mod getEvents;
use getEvents::getEvents::fetch;

#[allow(unused)]

use sqlx::postgres::{PgPoolOptions,PgRow};
use sqlx::{FromRow,Row};
use ethers::{
    providers::{Provider, Http, Middleware},
    types::{Address, Log, BlockNumber, Filter}, prelude::account::Sort, utils::hex::{self, encode, ToHex},
};
use ethcontract::{contract, contract::Event, web3::types::{H160, H256}};
use ethers::core::types::Chain;
use ethers::etherscan::Client;
use ethers::etherscan::account::TxListParams;
mod nft;
use nft::nft_data::getNftData;



#[tokio::main]
async fn main() -> eyre::Result<()> {


    getNftData().await;

    
    println!("all okay");
    Ok(())
}


