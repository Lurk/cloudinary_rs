mod access_mode;
mod allowed_headers;
mod background_removal;
mod categorizations;
mod delivery_type;
pub mod moderation;
mod options;
mod raw_convert;
mod resource_type;
mod responsive_breakpoints;
pub mod result;

use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client, Url};
use result::DestroyResult;
use sha1::{Digest, Sha1};
use std::collections::BTreeSet;
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub use self::result::UploadResult;
pub use self::{
    access_mode::AccessModes, allowed_headers::AllowedHeaders,
    background_removal::BackgroundRemoval, categorizations::Categorizations,
    delivery_type::DeliveryType, moderation::Moderation, options::OptionalParameters,
    resource_type::ResourceTypes, responsive_breakpoints::ResponsiveBreakpoints,
};

pub struct Upload {
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

pub enum Source {
    Path(PathBuf),
    Url(Url),
    DataUrl(String),
}

impl Upload {
    pub fn new(api_key: String, cloud_name: String, api_secret: String) -> Self {
        Upload {
            api_key,
            api_secret,
            cloud_name,
        }
    }

    /// Uploads an image
    ///
    /// ```rust
    /// use std::collections::BTreeSet;
    /// use cloudinary::upload::{Source, Upload, OptionalParameters};
    ///
    /// let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
    /// let options = BTreeSet::from([OptionalParameters::PublicId("file.jpg".to_string())]);
    /// let result = upload.image(Source::Path("./image.jpg".into()), &options);
    /// ```
    pub async fn image(
        &self,
        src: Source,
        options: &BTreeSet<OptionalParameters>,
    ) -> Result<UploadResult> {
        let client = Client::new();
        let file = match src {
            Source::Path(path) => prepare_file(&path).await?,
            Source::Url(url) => Part::text(url.as_str().to_string()),
            Source::DataUrl(base64) => Part::text(base64),
        };
        let multipart = self.build_form(options).part("file", file);
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

    /// destroy the asset by public id.
    ///
    /// ```rust
    /// use cloudinary::upload::{Source, Upload};
    /// let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
    /// let result = upload.destroy("image");
    /// ```
    pub async fn destroy<IS>(&self, public_id: IS) -> Result<DestroyResult>
    where
        IS: Into<String> + Clone,
    {
        let client = Client::new();

        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/destroy",
            self.cloud_name
        );
        let response = client
            .post(&url)
            .multipart(
                self.build_form(&BTreeSet::from([OptionalParameters::PublicId(
                    public_id.clone().into(),
                )])),
            )
            .send()
            .await
            .context(format!("destroy {}", public_id.into()))?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text).context(format!("failed to parse:\n\n {}", text))?;
        Ok(json)
    }

    fn build_form(&self, options: &BTreeSet<OptionalParameters>) -> Form {
        let mut form = Form::new();
        let mut hasher = Sha1::new();
        let timestamp = Utc::now().timestamp_millis().to_string();

        for option in options {
            let (key, value) = option.get_pair();
            if key != "resource_type" {
                hasher.update(option.to_string());
                hasher.update("&");
            };

            form = form.text(key, value);
        }

        hasher.update(format!("timestamp={}{}", timestamp, self.api_secret));

        form = form.text("signature", format!("{:x}", hasher.finalize()));
        form = form.text("api_key", self.api_key.clone());
        form = form.text("timestamp", timestamp.clone());

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
