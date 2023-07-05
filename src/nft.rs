pub mod nft_data{

    
    use std::sync::Arc;

    use ethers::{
        contract::Contract,
        providers::{Provider},
        types::{Address, U256,H160},
         abi::Abi
    };
    use serde_json::Value;
    
    
pub async fn getNftData(address:String,r_abi:&str) -> Result<(), Box<dyn std::error::Error>> {

    println!("Running nft");

        // Replace with your Ethereum node URL
        let rpc_url = "https://lingering-delicate-choice.discover.quiknode.pro/68f9e3726efe97ee2b6a7c8417f6f5d12ab713c6/";
        let provider = Provider::try_from(rpc_url)?;
        
        // NFT contract address
        let contract_address:H160 = address.parse()?;

        let abi: Abi = serde_json::from_str(r_abi).unwrap();

    let contract_instance=Contract::new(contract_address,abi,Arc::new(provider.clone()));

    // NFT token ID
    let token_id = U256::from(2210); // Replace with the actual token ID

    let owner: String = contract_instance
    .method::<_, String>("name", ())?
    .call()
    .await?;


    let symbol: String = contract_instance
    .method::<_, String>("symbol", ())?
    .call()
    .await?.to_string();

    let owner_token = contract_instance
    .method::<_, Address>("ownerOf", token_id)?
    .call()
    .await?;

    let token_uri = contract_instance
    .method::<_, String>("tokenURI", token_id)?
    .call()
    .await?;

    println!("NFT name: {:?}", owner);
    println!("NFT symbol: {:?}", symbol);
    println!("NFT owner of token {}: {:?}",token_id, owner_token);
    println!("Token URIr of token {}: {:?}",token_id, token_uri);
    // let _ = get_cid("https://ipfs.io/ipfs/bafkreieulfdm7miks2pgcs45rt4sv2s7bg775lt4acujl4l2unc5tn5hnu".to_string()).await;


    Ok(())
}



//function to read metadata
async fn get_cid(token_uri:String) -> Result<(), reqwest::Error> {
  println!("Reading metadata:");
    let body = reqwest::get(token_uri)
    .await?.text().await?;

    let v:Value=serde_json::from_str(&body).unwrap();
    println!("Name:{}",v["name"]);
    println!("--------------------------------------------------------------------------------");
    println!("chainID:{}",v["source"]["chainId"]);
    println!("--------------------------------------------------------------------------------");
    println!("Origin:{}",v["origin"]);
    println!("--------------------------------------------------------------------------------");

    Ok(())
}



}

