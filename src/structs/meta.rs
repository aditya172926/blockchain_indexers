use serde::Serialize;

#[derive(Serialize, Clone,Debug)]
pub struct MetaStruct {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    // #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    // pub title: Option<String>,
    // #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    // pub media: Option<String>,
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