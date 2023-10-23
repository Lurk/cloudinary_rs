//! [![codecov](https://codecov.io/gh/Lurk/cloudinary_rs/branch/main/graph/badge.svg?token=K8H5DLTSX4)](https://codecov.io/gh/Lurk/cloudinary_rs)
//! [![crates.io](https://img.shields.io/crates/v/cloudinary.svg)](https://crates.io/crates/cloudinary)
//! [![Released API docs](https://docs.rs/cloudinary/badge.svg)](https://docs.rs/cloudinary)
//!
//! At the moment, there is only half-backed upload and transformation functionality, but if you need more, please let
//! me know.
//!
//! # Upload an image
//!
//! ```rust
//! use cloudinary::{Source, Cloudinary};
//! use cloudinary::upload::{UploadOptions};
//! let options = UploadOptions::new().set_public_id("file.jpg".to_string());
//! let cloudinary = Cloudinary::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
//! let result = cloudinary.upload_image(Source::Path("./image.jpg".into()), &options);
//! ```
//!
//! # Transform an image
//!
//! ```rust
//! use cloudinary::transformation::{
//!     Transformations::Resize, resize_mode::ResizeMode::ScaleByWidth, Image, aspect_ratio::AspectRatio
//! };
//!
//! let image = Image::new("test".into(), "path/name.png".into())
//!     .add_transformation(Resize(ScaleByWidth{ width:100, ar: None, liquid:None}));
//! assert_eq!(
//!     image.to_string(),
//!     "https://res.cloudinary.com/test/image/upload/c_scale,w_100/path/name.png"
//! );
//! ```
//!
//! # Get Image from URL
//!
//! Unofficial api. This is not supported by Cloudinary, and can break at any time.
//! Officially you should use public_id that you get from upload.
//!
//! (Support)[https://support.cloudinary.com/hc/en-us/community/posts/360006941639-How-to-programmatically-retrieve-public-id-from-URL-]
//!
//! ```rust
//! use cloudinary::transformation::Image;
//! use url::Url;
//! let image = Image::try_from(
//!     Url::parse("https://res.cloudinary.com/test/image/upload/path/name.png").unwrap()
//! ).unwrap();
//! assert_eq!(image.to_string(), "https://res.cloudinary.com/test/image/upload/path/name.png");
//! ```
//!
//! # Get a list of all assets with a given tag
//! ```rust
//! # async fn tags(){
//! use cloudinary::tags::get_tags;
//! let tags = get_tags("cloud_name".into(), "tag_name".into()).await;
//! # }
//!
//! ```
//!
//! # Minimum supported Rust version
//!
//! The minimum supported Rust version for this crate is 1.65
//!
pub mod tags;
pub mod transformation;
pub mod upload;

use anyhow::{Context, Result};
use chrono::Utc;
use itertools::Itertools;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client, Url};
use sha1::{Digest, Sha1};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use upload::{result::UploadResult, UploadOptions};

pub struct Cloudinary {
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

pub enum Source {
    Path(PathBuf),
    Url(Url),
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
    /// use cloudinary::{Source, Cloudinary};
    /// use cloudinary::upload::{UploadOptions};
    /// let options = UploadOptions::new().set_public_id("file.jpg".to_string());
    /// let cloudinary = Cloudinary::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
    /// let result = cloudinary.upload_image(Source::Path("./image.jpg".into()), &options);
    /// ```
    pub async fn upload_image(
        &self,
        src: Source,
        options: &UploadOptions<'_>,
    ) -> Result<UploadResult> {
        let client = Client::new();
        let file = match src {
            Source::Path(path) => prepare_file(&path).await?,
            Source::Url(url) => Part::text(url.as_str().to_string()),
        };
        let multipart = self.build_form_data(options).part("file", file);
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            self.cloud_name
        );
        let response = client
            .post(&url)
            .multipart(multipart)
            .send()
            .await
            .context(format!("upload to {}", url))?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text).context(format!("failed to parse:\n\n {}", text))?;
        Ok(json)
    }

    fn build_form_data(&self, options: &UploadOptions) -> Form {
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

async fn prepare_file(src: &PathBuf) -> Result<Part> {
    let file = File::open(&src).await?;

    let filename = src.file_name().unwrap().to_string_lossy().into_owned();

    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);
    Ok(Part::stream(file_body)
        .file_name(filename)
        .mime_str("image/*")?)
}

#[cfg(test)]
mod tests;
