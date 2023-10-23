use log::{debug, error, info, warn};
use serde_json::Value;
use std::fs;

use crate::structs::extract::{Db, DbEnv};

pub async fn utils_db(env: String) -> Db {
    let f = std::fs::File::open(String::from("config/db.yml")).expect("Could not open file.");
    let db_config: DbEnv = serde_yaml::from_reader(f).expect("Could not read values.");

    if env == String::from("PROD") {
        return db_config.prod.clone();
    } else {
        return db_config.dev.clone();
    }
}
