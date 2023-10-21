use web3::contract::ens::Ens;

use crate::structs::index::{MethodParam, Meta};
use crate::structs::meta::{MetaIndexedStruct, self, MetaDataStruct};
use crate::structs::transactions::TransactionIndexed;
#[derive(Debug)]
struct EnsMeta {
   name:String,
   content:String,
   owner:String,
   duration:String,
   secret:String,
   resolver:String,
   data:String,
   reverseRecord:String,
   ownerControlledFuses:String,
}

pub fn handler(transaction_indexed:&TransactionIndexed)->Option<MetaDataStruct>{
   if transaction_indexed.method.name=="register" {
      let meta_raw: EnsMeta = EnsMeta{
         name:transaction_indexed.method.params[0].value.clone(),
         content:format!("This ens handle is owned by {} ", transaction_indexed.method.params[1].value.clone()),
         owner:transaction_indexed.method.params[1].value.clone(),
         duration:transaction_indexed.method.params[2].value.clone(),
         secret:transaction_indexed.method.params[3].value.clone(),
         resolver:transaction_indexed.method.params[4].value.clone(),
         data:transaction_indexed.method.params[5].value.clone(),
         reverseRecord:transaction_indexed.method.params[6].value.clone(),
         ownerControlledFuses:transaction_indexed.method.params[7].value.clone(),
      };
      let meta_indexed:MetaIndexedStruct = MetaIndexedStruct { id:Some(meta_raw.name.clone()) , owner:Some(meta_raw.owner), title:Some(format!("{}.eth",meta_raw.name.clone()))  };
      println!("\n\n\nMeta indexed {:?} \n\n\n", meta_indexed);
      let meta_data:MetaDataStruct = MetaDataStruct{
         modified:meta_indexed
      };
      return Some(meta_data);
   }
   else{
      return None;
   }
   
   
}