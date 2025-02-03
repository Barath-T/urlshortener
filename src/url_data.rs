use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlData {
    #[serde(rename = "_id")]
    pub short_url: String,
    pub original_url: String,
    pub uses_left: Option<i32>,
    pub expiration_date: String,
}
