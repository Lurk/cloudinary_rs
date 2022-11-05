pub mod upload;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client};
use serde::{de, Deserialize, Deserializer};
use sha1::{Digest, Sha1};
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use upload::UploadOptions;

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
    pub original_filename: String,
    pub api_key: String,
}

pub struct Cloudinary {
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

impl Cloudinary {
    pub fn new(api_key: String, cloud_name: String, api_secret: String) -> Self {
        Cloudinary {
            api_key,
            api_secret,
            cloud_name,
        }
    }

    /// uploads an image
    /// ```rust
    /// use cloudinary::{Cloudinary, upload::UploadOptions};
    /// let options = UploadOptions::new().set_public_id("file.jpg".to_string());
    /// let cloudinary = Cloudinary::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
    /// let result = cloudinary.upload_image("./image.jpg".to_string(), options);
    /// ```
    pub async fn upload_image(&self, src: String, options: UploadOptions<'_>) -> Response {
        let client = Client::new();
        match prepare_file(&src).await {
            Ok(file) => {
                let multipart = self.build_form_data(options).part("file", file);
                let response = client
                    .post(format!(
                        "https://api.cloudinary.com/v1_1/{}/image/upload",
                        self.cloud_name
                    ))
                    .multipart(multipart)
                    .send()
                    .await;
                match response {
                    Ok(r) => match r.json::<Response>().await {
                        Ok(val) => val,
                        Err(e) => panic!("Uh oh! Something unexpected happened: {:?}", e),
                    },
                    Err(e) => panic!("Uh oh! Something unexpected happened: {:?}", e),
                }
            }
            Err(err) => panic!("Uh oh! Something unexpected happened: {:?}", err),
        }
    }

    fn build_form_data(&self, options: UploadOptions) -> Form {
        let mut map = options.get_map();
        let resource_type = map.remove("resource_type");
        let timestamp = Utc::now().timestamp_millis().to_string();

        let mut form = Form::new()
            .text("api_key", self.api_key.clone())
            .text("timestamp", timestamp.clone());

        if let Some(value) = resource_type {
            form = form.text("resource_type", value);
        }

        let str = map.iter().map(|(k, v)| format!("{k}={v}")).join("&");
        let mut hasher = Sha1::new();
        if !str.is_empty() {
            hasher.update(str);
            hasher.update("&");
        }
        hasher.update(format!("timestamp={}{}", timestamp, self.api_secret));
        let signature = hasher.finalize();

        form = form.text("signature", format!("{:x}", signature));
        for (k, v) in map.iter() {
            form = form.text(k.clone(), v.clone());
        }
        form
    }
}

async fn prepare_file(src: &str) -> Result<Part, Box<dyn std::error::Error>> {
    let file = File::open(&src).await?;

    let filename = Path::new(src)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);
    Ok(Part::stream(file_body)
        .file_name(filename)
        .mime_str("image/*")?)
}
