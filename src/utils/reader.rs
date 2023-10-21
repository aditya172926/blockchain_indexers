use serde_yaml::{self};
use crate::structs::{extract::{Config}, meta};

pub fn utils_config(contract_slug:String)->Config{
    let path: String=format!("./src/schema/{}.yml",contract_slug);
    let f = std::fs::File::open(path).expect("Could not open file.");
    let meta_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("Meta Config:{:?}\n", meta_config);

    return meta_config;
}
