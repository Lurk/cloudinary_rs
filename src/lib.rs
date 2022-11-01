mod upload;

use chrono::prelude::*;
use itertools::Itertools;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client};
use sha1::{Digest, Sha1};
use std::path::Path;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use upload::UploadOptions;

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
    pub async fn upload_image(
        self,
        src: String,
        options: UploadOptions<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let file = prepare_file(&src).await?;
        let multipart = self.build_form_data(options).part("file", file);
        let res = client
            .post(format!(
                "https://api.cloudinary.com/v1_1/{}/image/upload",
                self.cloud_name
            ))
            .multipart(multipart)
            .send()
            .await;

        Ok(())
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
