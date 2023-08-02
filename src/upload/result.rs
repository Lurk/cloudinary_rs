use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};

fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,      // Required for S::from_str...
    S::Err: Display, // Required for .map_err(de::Error::custom)
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

#[derive(Clone, Deserialize, Debug)]
pub struct Error {
    pub error: Message,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    pub message: String,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum UploadResult {
    Success(Box<Response>),
    Error(Box<Error>),
}

#[derive(Clone, Deserialize, Debug)]
pub struct Response {
    pub asset_id: String,
    pub public_id: String,
    pub version: usize,
    pub version_id: String,
    pub signature: String,
    pub width: usize,
    pub height: usize,
    pub format: String,
    pub resource_type: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub bytes: usize,
    pub r#type: String,
    pub etag: String,
    pub placeholder: bool,
    pub url: String,
    pub secure_url: String,
    pub folder: String,
    pub overwritten: Option<bool>,
    pub original_filename: Option<String>,
    pub original_extension: Option<String>,
    pub api_key: String,
}
