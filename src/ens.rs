// use ethcontract::{prelude::*, web3::types::H256,log::LogFilterBuilder};
// use  web3::{contract::ens::Ens, api::Namespace};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Create an instance of the Ethereum transport
//     let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/d0ff55026e4f4547b9f334497888fc07")?;
//     let web3 = web3::Web3::new(&transport);

//     let _ = ens_name(&transport).await;
//     Ok(())
// }

// async fn ens_name(transport:&Http) ->  eyre::Result<(),Box<dyn Error>>{


//     let accounts = vec![
//         "0xE7ECb3219Be5Bc42caA7d77966f91F68a0eBfFf2".parse()?,
//         "0x111F530216fBB0377B4bDd4d303a465a1090d09d".parse()?
//   ];

//   for acc in accounts{

// //Create ENS instance
//       let ens = Ens::new(transport);

// //Fetch ENS name
//       let  name=ens.canonical_name(acc).await;
//       match name{
//           Ok(string)=>println!("{}",string),
//           Err(error)=>println!("No valid ENS name found!")
//       };
//   }
  

//     Ok(())
// }

pub mod ens{
    
    use  web3::{contract::ens::Ens, api::Namespace};
    use std::error::Error;

    pub async fn fetch_ens_name()-> Result<(), Box<dyn Error>>{

    // Create an instance of the Ethereum transport
    let transport = web3::transports::Http::new("https://mainnet.infura.io/v3/d0ff55026e4f4547b9f334497888fc07")?;
    let web3 = web3::Web3::new(&transport);

    let accounts = vec![
        "0xE7ECb3219Be5Bc42caA7d77966f91F68a0eBfFf2".parse()?,
        "0x111F530216fBB0377B4bDd4d303a465a1090d09d".parse()?
  ];

  for acc in accounts{

//Create ENS instance
      let ens = Ens::new(&transport);

//Fetch ENS name
      let  name=ens.canonical_name(acc).await;
      match name{
          Ok(string)=>println!("{}",string),
          Err(error)=>println!("No valid ENS name found!")
      };
  }
  

    Ok(())
}
}
    
