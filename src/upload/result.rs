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
    Response(Box<Response>),
    /// New* accounts get response in this format by default
    /// * unfortunately I was not able to find out what exactly "new" means
    ResponseWithImageMetadata(Box<ResponseWithImageMetadata>),
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

#[derive(Clone, Deserialize, Debug)]
pub struct ImageMetadata {
    #[serde(rename(serialize = "JFIFVersion", deserialize = "JFIFVersion"))]
    pub jfif_version: Option<String>,
    #[serde(rename(serialize = "ResolutionUnit", deserialize = "ResolutionUnit"))]
    pub resolution_unit: Option<String>,
    #[serde(rename(serialize = "XResolution", deserialize = "XResolution"))]
    pub x_resolution: Option<String>,
    #[serde(rename(serialize = "YResolution", deserialize = "YResolution"))]
    pub y_resolution: Option<String>,
    #[serde(rename(serialize = "Colorspace", deserialize = "Colorspace"))]
    pub colorspace: Option<String>,
    #[serde(rename(serialize = "DPI", deserialize = "DPI"))]
    pub dpi: Option<String>,
}

/// https://cloudinary.com/documentation/image_upload_api_reference#upload_response
#[derive(Clone, Deserialize, Debug)]
pub struct ResponseWithImageMetadata {
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
    pub asset_folder: String,
    pub overwritten: Option<bool>,
    pub display_name: String,
    pub image_metadata: Option<ImageMetadata>,
    pub illustration_score: Option<f64>,
    pub semi_transparent: Option<bool>,
    pub grayscale: Option<bool>,
    pub api_key: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DestroyResult {
    pub result: String,
}
