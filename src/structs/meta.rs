use serde::Serialize;

use super::transactions::TransactionIndexed;

#[derive(Serialize, Clone, Debug)]
pub struct Meta {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    // #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    // pub content: Option<String>,
    // #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    // pub context: Option<String>,
    // #[serde(rename = "ipfs", skip_serializing_if = "Option::is_none")]
    // pub ipfs: Option<String>,
    // #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    // pub created_at: Option<String>,
    // #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    // pub updated_at: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct MetaData {
    // #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    // raw:Option<>,
    pub modified: Meta,
}

#[derive(Debug, Serialize)]
pub struct MetaIndexed {
    pub metaOwner: String,
    pub metaId: String,
    pub slug: String,
    pub data: MetaData,
    pub createdAt: String,
    pub updatedAt: String,
    pub sources: Vec<TransactionIndexed>,
}
