use crate::structs::{extract::Config, meta};
use log::info;
use serde_yaml::{self};

pub fn utils_config(contract_slug: String) -> Config {
    let slug = contract_slug[1..contract_slug.len() - 1].to_string();
    let path: String = format!("./src/schema/{}.yml", slug);
    let f = std::fs::File::open(path).expect("Could not open file.");
    let meta_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    return meta_config;
}
