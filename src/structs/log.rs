use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Log {
    pub slug: String,
    pub docsLength: String,
    pub blockStart: String,
    pub blockEnd: String,
}
