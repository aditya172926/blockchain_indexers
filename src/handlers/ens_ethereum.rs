use crate::structs::index::MethodParam;
use crate::structs::meta::MetaStruct;

pub fn handler(method_params:&Vec<MethodParam>){
     println!("\n\n\nmethod_params {:?} \n\n\n",method_params);
     let meta:MetaStruct = MetaStruct{
        id:Some(method_params[0].value.clone()),
        owner:Some(method_params[1].value.clone()),
        // title: Some(""),
        // media: None(),
        // content: None(),
        // context: todo!(),
        // ipfs: todo!(),
        // created_at: todo!(),
        // updated_at: todo!(),
        
     };
     println!("\n\n\n metaStruct {:?}\n\n\n\n", meta)
}