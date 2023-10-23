use crate::structs::{extract::Schema, meta};
use log::info;
use serde_yaml::{self};

pub fn utils_schema(slug: String) -> Schema {
    let path: String = format!("./src/schema/{}.yml", slug);
    info!("{}", path);
    let f = std::fs::File::open(path).expect("Could not open file.");
    let schema: Schema = serde_yaml::from_reader(f).expect("Could not read values.");

    return schema;
}
