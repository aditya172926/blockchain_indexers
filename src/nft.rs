pub mod nft_data{

    
    use std::sync::Arc;

    use ethers::{
        contract::Contract,
        providers::{Provider},
        types::{Address, U256,H160},
         abi::Abi
    };
    use serde_json::Value;
    
    
pub async fn getNftData() -> Result<(), Box<dyn std::error::Error>> {

    println!("Running nft");

        // Replace with your Ethereum node URL
        let rpc_url = "https://lingering-delicate-choice.discover.quiknode.pro/68f9e3726efe97ee2b6a7c8417f6f5d12ab713c6/";
       let provider = Provider::try_from(rpc_url)?;
        
        // NFT contract address
        let contract_address:H160 = "0xB6a37b5d14D502c3Ab0Ae6f3a0E058BC9517786e".parse()?;

        let abi: Abi = serde_json::from_str(r#"[
            {
              "inputs": [
                { "internalType": "string", "name": "_name", "type": "string" },
                { "internalType": "string", "name": "_symbol", "type": "string" },
                { "internalType": "uint16", "name": "maxSupply_", "type": "uint16" }
              ],
              "stateMutability": "nonpayable",
              "type": "constructor"
            },
            { "inputs": [], "name": "AlreadyMinted", "type": "error" },
            { "inputs": [], "name": "BeanAddressNotSet", "type": "error" },
            { "inputs": [], "name": "InvalidRecipient", "type": "error" },
            { "inputs": [], "name": "InvalidRedeemer", "type": "error" },
            { "inputs": [], "name": "InvalidTokenId", "type": "error" },
            { "inputs": [], "name": "NoMoreTokenIds", "type": "error" },
            { "inputs": [], "name": "NotAllowedByRegistry", "type": "error" },
            { "inputs": [], "name": "NotMinted", "type": "error" },
            { "inputs": [], "name": "RedeemBeanNotOpen", "type": "error" },
            { "inputs": [], "name": "RegistryNotSet", "type": "error" },
            { "inputs": [], "name": "Unauthorized", "type": "error" },
            { "inputs": [], "name": "UnsafeRecipient", "type": "error" },
            { "inputs": [], "name": "WrongFrom", "type": "error" },
            { "inputs": [], "name": "ZeroAddress", "type": "error" },
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "owner",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "spender",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "uint256",
                  "name": "id",
                  "type": "uint256"
                }
              ],
              "name": "Approval",
              "type": "event"
            },
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "owner",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "operator",
                  "type": "address"
                },
                {
                  "indexed": false,
                  "internalType": "bool",
                  "name": "approved",
                  "type": "bool"
                }
              ],
              "name": "ApprovalForAll",
              "type": "event"
            },
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "to",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "uint256",
                  "name": "tokenId",
                  "type": "uint256"
                },
                {
                  "indexed": true,
                  "internalType": "uint256",
                  "name": "beanId",
                  "type": "uint256"
                }
              ],
              "name": "BeanRedeemed",
              "type": "event"
            },
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "previousOwner",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "newOwner",
                  "type": "address"
                }
              ],
              "name": "OwnershipTransferred",
              "type": "event"
            },
            {
              "anonymous": false,
              "inputs": [
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "from",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "address",
                  "name": "to",
                  "type": "address"
                },
                {
                  "indexed": true,
                  "internalType": "uint256",
                  "name": "id",
                  "type": "uint256"
                }
              ],
              "name": "Transfer",
              "type": "event"
            },
            {
              "inputs": [],
              "name": "MAX_SUPPLY",
              "outputs": [{ "internalType": "uint16", "name": "", "type": "uint16" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "operator", "type": "address" },
                { "internalType": "uint256", "name": "tokenId", "type": "uint256" }
              ],
              "name": "approve",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "owner", "type": "address" }
              ],
              "name": "balanceOf",
              "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
              "name": "getApproved",
              "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "", "type": "address" },
                { "internalType": "address", "name": "", "type": "address" }
              ],
              "name": "isApprovedForAll",
              "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "isRegistryActive",
              "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "name",
              "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "operatorFilteringEnabled",
              "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "owner",
              "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [{ "internalType": "uint256", "name": "id", "type": "uint256" }],
              "name": "ownerOf",
              "outputs": [
                { "internalType": "address", "name": "owner", "type": "address" }
              ],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "realOwner",
              "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "to", "type": "address" },
                { "internalType": "uint256[]", "name": "beanIds", "type": "uint256[]" }
              ],
              "name": "redeemBeans",
              "outputs": [
                { "internalType": "uint256[]", "name": "", "type": "uint256[]" }
              ],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "redeemInfo",
              "outputs": [
                { "internalType": "bool", "name": "redeemBeanOpen", "type": "bool" },
                { "internalType": "address", "name": "beanAddress", "type": "address" }
              ],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "registryAddress",
              "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "renounceOwnership",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
                { "internalType": "uint256", "name": "salePrice", "type": "uint256" }
              ],
              "name": "royaltyInfo",
              "outputs": [
                { "internalType": "address", "name": "", "type": "address" },
                { "internalType": "uint256", "name": "", "type": "uint256" }
              ],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "from", "type": "address" },
                { "internalType": "address", "name": "to", "type": "address" },
                { "internalType": "uint256", "name": "id", "type": "uint256" }
              ],
              "name": "safeTransferFrom",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "from", "type": "address" },
                { "internalType": "address", "name": "to", "type": "address" },
                { "internalType": "uint256", "name": "id", "type": "uint256" },
                { "internalType": "bytes", "name": "data", "type": "bytes" }
              ],
              "name": "safeTransferFrom",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "operator", "type": "address" },
                { "internalType": "bool", "name": "approved", "type": "bool" }
              ],
              "name": "setApprovalForAll",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "string", "name": "baseURI", "type": "string" }
              ],
              "name": "setBaseURI",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "string", "name": "baseURIPermanent", "type": "string" }
              ],
              "name": "setBaseURIPermanent",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                {
                  "internalType": "address",
                  "name": "contractAddress",
                  "type": "address"
                }
              ],
              "name": "setBeanAddress",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "receiver", "type": "address" },
                { "internalType": "uint96", "name": "feeNumerator", "type": "uint96" }
              ],
              "name": "setDefaultRoyalty",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "bool", "name": "_isRegistryActive", "type": "bool" }
              ],
              "name": "setIsRegistryActive",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "uint256[]", "name": "tokenIds", "type": "uint256[]" }
              ],
              "name": "setIsUriPermanent",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "string", "name": "_newName", "type": "string" },
                { "internalType": "string", "name": "_newSymbol", "type": "string" }
              ],
              "name": "setNameAndSymbol",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [{ "internalType": "bool", "name": "value", "type": "bool" }],
              "name": "setOperatorFilteringEnabled",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "bool", "name": "_redeemBeanOpen", "type": "bool" }
              ],
              "name": "setRedeemBeanState",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                {
                  "internalType": "address",
                  "name": "_registryAddress",
                  "type": "address"
                }
              ],
              "name": "setRegistryAddress",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "uint256", "name": "tokenId", "type": "uint256" },
                { "internalType": "address", "name": "receiver", "type": "address" },
                { "internalType": "uint96", "name": "feeNumerator", "type": "uint96" }
              ],
              "name": "setTokenRoyalty",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "bytes4", "name": "interfaceId", "type": "bytes4" }
              ],
              "name": "supportsInterface",
              "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "symbol",
              "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "uint256", "name": "tokenId", "type": "uint256" }
              ],
              "name": "tokenURI",
              "outputs": [{ "internalType": "string", "name": "", "type": "string" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [],
              "name": "totalSupply",
              "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
              "stateMutability": "view",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "from", "type": "address" },
                { "internalType": "address", "name": "to", "type": "address" },
                { "internalType": "uint256", "name": "id", "type": "uint256" }
              ],
              "name": "transferFrom",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "newOwner", "type": "address" }
              ],
              "name": "transferLowerOwnership",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "newOwner", "type": "address" }
              ],
              "name": "transferOwnership",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            },
            {
              "inputs": [
                { "internalType": "address", "name": "newRealOwner", "type": "address" }
              ],
              "name": "transferRealOwnership",
              "outputs": [],
              "stateMutability": "nonpayable",
              "type": "function"
            }
          ]
          "#)?;

    let contract_instance=Contract::new(contract_address,abi,Arc::new(provider.clone()));

    // NFT token ID
    let token_id = U256::from(2210); // Replace with the actual token ID

    // let owner=contract_instance.call("ownerOf",(token_id,),None).await?;
    let init_value: String = contract_instance
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

    println!("NFT name: {:?}", init_value);
    println!("NFT symbol: {:?}", symbol);
    println!("NFT owner of token {}: {:?}",token_id, owner_token);
    println!("NFT owner of token {}: {:?}",token_id, token_uri);
    let _ = get_cid(token_uri).await;


    Ok(())
}



//function to read metadata
async fn get_cid(token_uri:String) -> Result<(), reqwest::Error> {
  println!("Reading metadata:");
    let body = reqwest::get(token_uri)
    .await?.text().await?;

    let v:Value=serde_json::from_str(&body).unwrap();
    println!("{}",v["image"]);

    Ok(())
}



}

