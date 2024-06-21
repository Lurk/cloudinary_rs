mod access_mode;
mod allowed_headers;
mod background_removal;
mod categorizations;
mod data_types;
mod delivery_type;
mod raw_convert;
mod resource_type;
mod responsive_breakpoints;
pub mod result;

use anyhow::{Context, Result};
use chrono::Utc;
use itertools::Itertools;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client, Url};
use sha1::{Digest, Sha1};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use self::result::UploadResult;
use self::{
    access_mode::AccessModes, allowed_headers::AllowedHeaders,
    background_removal::BackgroundRemoval, categorizations::Categorizations,
    data_types::Coordinates, data_types::DataType, delivery_type::DeliveryType,
    raw_convert::RawConvert, resource_type::ResourceTypes,
    responsive_breakpoints::ResponsiveBreakpoints,
};

pub struct Upload {
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

pub enum Source {
    Path(PathBuf),
    Url(Url),
    Base64(String),
}

impl Upload {
    pub fn new(api_key: String, cloud_name: String, api_secret: String) -> Self {
        Upload {
            api_key,
            api_secret,
            cloud_name,
        }
    }

    /// uploads an image
    /// ```rust
    /// use cloudinary::upload::{UploadOptions, Source, Upload};
    /// let options = UploadOptions::new().set_public_id("file.jpg".to_string());
    /// let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
    /// let result = upload.image(Source::Path("./image.jpg".into()), &options);
    /// ```
    pub async fn image(&self, src: Source, options: &UploadOptions<'_>) -> Result<UploadResult> {
        let client = Client::new();
        let file = match src {
            Source::Path(path) => prepare_file(&path).await?,
            Source::Url(url) => Part::text(url.as_str().to_string()),
            Source::Base64(base64) => Part::text(base64),
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

#[derive(Debug)]
pub struct UploadOptions<'entry_key_lifetime> {
    inner: BTreeMap<&'entry_key_lifetime str, DataType>,
}

impl<'entry_key_lifetime> UploadOptions<'entry_key_lifetime> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    fn get_string(&self, key: &str) -> Option<String> {
        if let Some(DataType::String(value)) = self.inner.get(key) {
            return Some(value.clone());
        }
        None
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        if let Some(DataType::Boolean(value)) = self.inner.get(key) {
            return Some(*value);
        }
        None
    }

    fn add_to_hashmap(
        mut self,
        hashmap_name: &'entry_key_lifetime str,
        key: String,
        value: String,
    ) -> Self {
        if let Some(DataType::HashMap(context)) = self.inner.get_mut(hashmap_name) {
            context.insert(key, value);
        } else {
            self.inner.insert(
                hashmap_name,
                DataType::HashMap(HashMap::from([(key, value)])),
            );
        }

        self
    }

    fn remove_from_hashmap(mut self, hashmap: &str, key: &str) -> Self {
        if let Some(DataType::HashMap(context)) = self.inner.get_mut(hashmap) {
            context.remove(key);
            if context.is_empty() {
                self.inner.remove(hashmap);
            }
        }
        self
    }

    pub fn add_tags(mut self, tags: &[String]) -> Self {
        if let Some(DataType::HasSet(inner_tags)) = self.inner.get_mut("tags") {
            inner_tags.extend(tags.iter().cloned());
        } else {
            self.inner.insert(
                "tags",
                DataType::HasSet(HashSet::from_iter(tags.iter().cloned())),
            );
        }
        self
    }

    pub fn remove_tags(mut self, tags: &[String]) -> Self {
        if let Some(DataType::HasSet(inner_tags)) = self.inner.get_mut("tags") {
            for tag in tags {
                inner_tags.remove(tag);
            }
            if inner_tags.is_empty() {
                self.inner.remove("tags");
            }
        }
        self
    }

    pub fn get_tags(&self) -> Option<HashSet<String>> {
        if let Some(DataType::HasSet(inner_tags)) = self.inner.get("tags") {
            return Some(inner_tags.clone());
        }
        None
    }

    pub fn add_context(self, key: String, value: String) -> Self {
        self.add_to_hashmap("context", key, value)
    }

    pub fn remove_context(self, key: &str) -> Self {
        self.remove_from_hashmap("context", key)
    }

    pub fn get_context(&self, key: &String) -> Option<String> {
        if let Some(DataType::HashMap(context)) = self.inner.get("context") {
            return context.get(key).cloned();
        }
        None
    }

    pub fn add_metadata(self, key: String, value: String) -> Self {
        self.add_to_hashmap("metadata", key, value)
    }

    pub fn remove_metadata(self, key: &str) -> Self {
        self.remove_from_hashmap("metadata", key)
    }

    pub fn get_metadata(&self, key: &String) -> Option<String> {
        if let Some(DataType::HashMap(metadata)) = self.inner.get("metadata") {
            return metadata.get(key).cloned();
        }
        None
    }

    pub fn add_auto_tagging(mut self, auto_tagging: Option<f32>) -> Self {
        if let Some(value) = auto_tagging {
            self.inner
                .insert("auto_tagging", DataType::Float(value.clamp(0.0, 1.0)));
        } else {
            self.inner.remove("auto_tagging");
        }
        self
    }

    pub fn get_auto_tagging(&self) -> Option<f32> {
        if let Some(DataType::Float(auto_tagging)) = self.inner.get("auto_tagging") {
            return Some(*auto_tagging);
        }
        None
    }

    pub fn get_map(&self) -> BTreeMap<String, String> {
        self.inner.iter().fold(BTreeMap::new(), |mut acc, (k, v)| {
            acc.insert(k.to_string(), v.to_string());
            acc
        })
    }

    pub fn get_folder(&self) -> Option<String> {
        self.get_string("folder")
    }

    pub fn set_folder(mut self, value: String) -> Self {
        self.inner.insert("folder", DataType::String(value));
        self
    }

    pub fn get_upload_preset(&self) -> Option<String> {
        self.get_string("upload_preset")
    }

    pub fn set_upload_preset(mut self, value: String) -> Self {
        self.inner.insert("upload_preset", DataType::String(value));
        self
    }

    pub fn get_type(&self) -> Option<DeliveryType> {
        if let Some(DataType::DeliveryType(value)) = self.inner.get("type") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_type(mut self, value: DeliveryType) -> Self {
        self.inner.insert("type", DataType::DeliveryType(value));
        self
    }

    pub fn get_access_mode(&self) -> Option<AccessModes> {
        if let Some(DataType::AccessModes(value)) = self.inner.get("access_mode") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_access_mode(mut self, value: AccessModes) -> Self {
        self.inner
            .insert("access_mode", DataType::AccessModes(value));
        self
    }

    pub fn get_public_id(&self) -> Option<String> {
        self.get_string("public_id")
    }

    pub fn set_public_id(mut self, value: String) -> Self {
        self.inner.insert("public_id", DataType::String(value));
        self
    }

    pub fn get_use_filename(&self) -> Option<bool> {
        self.get_bool("use_filename")
    }

    pub fn set_use_filename(mut self, value: bool) -> Self {
        self.inner.insert("use_filename", DataType::Boolean(value));
        self
    }

    pub fn get_filename_override(&self) -> Option<String> {
        self.get_string("filename_override")
    }

    pub fn set_filename_override(mut self, value: String) -> Self {
        self.inner
            .insert("filename_override", DataType::String(value));
        self
    }

    pub fn get_resource_type(&self) -> Option<ResourceTypes> {
        if let Some(DataType::ResourceTypes(value)) = self.inner.get("resource_type") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_resource_type(mut self, value: ResourceTypes) -> Self {
        self.inner
            .insert("resource_type", DataType::ResourceTypes(value));
        self
    }

    pub fn get_discard_original_filename(&self) -> Option<bool> {
        self.get_bool("discard_original_filename")
    }

    pub fn set_discard_original_filename(mut self, value: bool) -> Self {
        self.inner
            .insert("discard_original_filename", DataType::Boolean(value));
        self
    }

    pub fn get_overwrite(&self) -> Option<bool> {
        self.get_bool("overwrite")
    }

    pub fn set_overwrite(mut self, value: bool) -> Self {
        self.inner.insert("overwrite", DataType::Boolean(value));
        self
    }

    pub fn get_colors(&self) -> Option<bool> {
        self.get_bool("colors")
    }

    pub fn set_colors(mut self, value: bool) -> Self {
        self.inner.insert("colors", DataType::Boolean(value));
        self
    }

    pub fn get_faces(&self) -> Option<bool> {
        self.get_bool("faces")
    }

    pub fn set_faces(mut self, value: bool) -> Self {
        self.inner.insert("faces", DataType::Boolean(value));
        self
    }

    pub fn get_quality_analysis(&self) -> Option<bool> {
        self.get_bool("quality_analysis")
    }

    pub fn set_quality_analysis(mut self, value: bool) -> Self {
        self.inner
            .insert("quality_analysis", DataType::Boolean(value));
        self
    }

    pub fn get_accessibility_analysis(&self) -> Option<bool> {
        self.get_bool("accessibility_analysis")
    }

    pub fn set_accessibility_analysis(mut self, value: bool) -> Self {
        self.inner
            .insert("accessibility_analysis", DataType::Boolean(value));
        self
    }

    pub fn get_cinemagraph_analysis(&self) -> Option<bool> {
        self.get_bool("cinemagraph_analysis")
    }

    pub fn set_cinemagraph_analysis(mut self, value: bool) -> Self {
        self.inner
            .insert("cinemagraph_analysis", DataType::Boolean(value));
        self
    }

    pub fn get_image_metadata(&self) -> Option<bool> {
        self.get_bool("image_metadata")
    }

    pub fn set_image_metadata(mut self, value: bool) -> Self {
        self.inner
            .insert("image_metadata", DataType::Boolean(value));
        self
    }

    pub fn get_phash(&self) -> Option<bool> {
        self.get_bool("phash")
    }

    pub fn set_phash(mut self, value: bool) -> Self {
        self.inner.insert("phash", DataType::Boolean(value));
        self
    }

    pub fn get_responsive_breakpoints(&self) -> Option<Vec<ResponsiveBreakpoints>> {
        if let Some(DataType::ResponsiveBreakpoints(value)) =
            self.inner.get("responsive_breakpoints")
        {
            return Some(value.clone());
        }
        None
    }

    pub fn set_responsive_breakpoints(mut self, value: Vec<ResponsiveBreakpoints>) -> Self {
        self.inner.insert(
            "responsive_breakpoints",
            DataType::ResponsiveBreakpoints(value),
        );
        self
    }

    pub fn get_categorization(&self) -> Option<Vec<Categorizations>> {
        if let Some(DataType::Categorization(value)) = self.inner.get("categorization") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_categorization(mut self, value: Vec<Categorizations>) -> Self {
        self.inner
            .insert("categorization", DataType::Categorization(value));
        self
    }

    pub fn get_detection(&self) -> Option<String> {
        self.get_string("detection")
    }

    pub fn set_detection(mut self, value: String) -> Self {
        self.inner.insert("detection", DataType::String(value));
        self
    }

    pub fn get_ocr(&self) -> Option<String> {
        self.get_string("ocr")
    }

    pub fn set_ocr(mut self, value: String) -> Self {
        self.inner.insert("ocr", DataType::String(value));
        self
    }

    pub fn get_eager(&self) -> Option<String> {
        self.get_string("eager")
    }

    pub fn set_eager(mut self, value: String) -> Self {
        self.inner.insert("eager", DataType::String(value));
        self
    }

    pub fn get_eager_async(&self) -> Option<bool> {
        self.get_bool("eager_async")
    }

    pub fn set_eager_async(mut self, value: bool) -> Self {
        self.inner.insert("eager_async", DataType::Boolean(value));
        self
    }

    pub fn get_eager_notification_url(&self) -> Option<String> {
        self.get_string("eager_notification_url")
    }

    pub fn set_eager_notification_url(mut self, value: String) -> Self {
        self.inner
            .insert("eager_notification_url", DataType::String(value));
        self
    }

    pub fn get_transformation(&self) -> Option<String> {
        self.get_string("transformation")
    }

    pub fn set_transformation(mut self, value: String) -> Self {
        self.inner.insert("transformation", DataType::String(value));
        self
    }

    pub fn get_format(&self) -> Option<String> {
        self.get_string("format")
    }

    pub fn set_format(mut self, value: String) -> Self {
        self.inner.insert("format", DataType::String(value));
        self
    }

    pub fn get_custom_coordinates(&self) -> Option<Coordinates> {
        if let Some(DataType::Coordinates(value)) = self.inner.get("custom_coordinates") {
            return Some(*value);
        }
        None
    }

    pub fn set_custom_coordinates(mut self, value: Coordinates) -> Self {
        self.inner
            .insert("custom_coordinates", DataType::Coordinates(value));
        self
    }

    pub fn get_face_coordinates(&self) -> Option<Vec<Coordinates>> {
        if let Some(DataType::FaceCoordinates(value)) = self.inner.get("face_coordinates") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_face_coordinates(mut self, value: Vec<Coordinates>) -> Self {
        self.inner
            .insert("face_coordinates", DataType::FaceCoordinates(value));
        self
    }

    pub fn get_background_removal(&self) -> Option<BackgroundRemoval> {
        if let Some(DataType::BackgroundRemoval(value)) = self.inner.get("background_removal") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_background_removal(mut self, value: BackgroundRemoval) -> Self {
        self.inner
            .insert("background_removal", DataType::BackgroundRemoval(value));
        self
    }

    pub fn get_raw_convert(&self) -> Option<RawConvert> {
        if let Some(DataType::RawConvert(value)) = self.inner.get("raw_convert") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_raw_convert(mut self, value: RawConvert) -> Self {
        self.inner
            .insert("raw_convert", DataType::RawConvert(value));
        self
    }

    pub fn get_allowed_formats(&self) -> Option<Vec<String>> {
        if let Some(DataType::VecOfString(value)) = self.inner.get("allowed_formats") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_allowed_formats(mut self, value: Vec<String>) -> Self {
        self.inner
            .insert("allowed_formats", DataType::VecOfString(value));
        self
    }

    pub fn get_async(&self) -> Option<bool> {
        self.get_bool("async")
    }

    pub fn set_async(mut self, value: bool) -> Self {
        self.inner.insert("async", DataType::Boolean(value));
        self
    }

    pub fn get_backup(&self) -> Option<bool> {
        self.get_bool("backup")
    }

    pub fn set_backup(mut self, value: bool) -> Self {
        self.inner.insert("backup", DataType::Boolean(value));
        self
    }

    pub fn get_callback(&self) -> Option<String> {
        self.get_string("callback")
    }

    pub fn set_callback(mut self, value: String) -> Self {
        self.inner.insert("callback", DataType::String(value));
        self
    }

    pub fn get_eval(&self) -> Option<String> {
        self.get_string("eval")
    }

    pub fn set_eval(mut self, value: String) -> Self {
        self.inner.insert("eval", DataType::String(value));
        self
    }

    pub fn get_headers(&self) -> Option<HashMap<AllowedHeaders, String>> {
        if let Some(DataType::AllowedHeaders(value)) = self.inner.get("headers") {
            return Some(value.clone());
        }
        None
    }

    pub fn set_headers(mut self, value: HashMap<AllowedHeaders, String>) -> Self {
        self.inner
            .insert("headers", DataType::AllowedHeaders(value));
        self
    }

    pub fn get_invalidate(&self) -> Option<bool> {
        self.get_bool("invalidate")
    }

    pub fn set_invalidate(mut self, value: bool) -> Self {
        self.inner.insert("invalidate", DataType::Boolean(value));
        self
    }

    pub fn get_notification_url(&self) -> Option<String> {
        self.get_string("notification_url")
    }

    pub fn set_notification_url(mut self, value: String) -> Self {
        self.inner
            .insert("notification_url", DataType::String(value));
        self
    }

    pub fn get_proxy(&self) -> Option<String> {
        self.get_string("proxy")
    }

    pub fn set_proxy(mut self, value: String) -> Self {
        self.inner.insert("proxy", DataType::String(value));
        self
    }

    pub fn get_return_delete_token(&self) -> Option<bool> {
        self.get_bool("return_delete_token")
    }

    pub fn set_return_delete_token(mut self, value: bool) -> Self {
        self.inner
            .insert("return_delete_token", DataType::Boolean(value));
        self
    }
}

impl Default for UploadOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::upload::access_mode::AccessModes;
    use crate::upload::delivery_type::DeliveryType;

    use super::UploadOptions;

    #[test]
    fn folder() {
        let params = UploadOptions::new().set_folder("folder".to_string());
        assert_eq!(params.get_folder(), Some("folder".to_string()));
    }
    #[test]
    fn upload_preset() {
        let params = UploadOptions::new().set_upload_preset("upload_preset".to_string());
        assert_eq!(
            params.get_upload_preset(),
            Some("upload_preset".to_string())
        );
    }
    #[test]
    fn r#type() {
        let params = UploadOptions::new().set_type(DeliveryType::Private);
        assert_eq!(params.get_type(), Some(DeliveryType::Private));
    }
    #[test]
    fn access_mode() {
        let params = UploadOptions::new().set_access_mode(AccessModes::Public);
        assert_eq!(params.get_access_mode(), Some(AccessModes::Public));
    }
    #[test]
    fn public_id() {
        let params = UploadOptions::new().set_public_id("public_id".to_string());
        assert_eq!(params.get_public_id(), Some("public_id".to_string()));
    }
    #[test]
    fn use_filename() {
        let params = UploadOptions::new().set_use_filename(false);
        assert_eq!(params.get_use_filename(), Some(false));
    }
    #[test]
    fn tags() {
        let mut params = UploadOptions::new();
        params = params.add_tags(&["foo".to_string(), "bar".to_string()]);
        assert_eq!(
            params.get_tags(),
            Some(HashSet::from(["foo".to_string(), "bar".to_string()]))
        );
        params = params.remove_tags(&["bar".to_string()]);
        assert_eq!(params.get_tags(), Some(HashSet::from(["foo".to_string()])));
        params = params.remove_tags(&["foo".to_string()]);
        assert_eq!(params.get_tags(), None);
    }
    #[test]
    fn context() {
        let mut params = UploadOptions::new();
        params = params.add_context("foo".to_string(), "context".to_string());
        assert_eq!(
            params.get_context(&"foo".to_string()),
            Some("context".to_string())
        );
        params = params.remove_context("foo");
        assert_eq!(params.get_context(&"foo".to_string()), None);
    }
    #[test]
    fn metadata() {
        let mut params = UploadOptions::new();
        params = params.add_metadata("foo".to_string(), "metadata".to_string());
        assert_eq!(
            params.get_metadata(&"foo".to_string()),
            Some("metadata".to_string())
        );
        params = params.remove_metadata("foo");
        assert_eq!(params.get_metadata(&"foo".to_string()), None);
    }
    #[test]
    fn auto_tagging() {
        let mut params = UploadOptions::new();
        params = params.add_auto_tagging(Some(0.5));
        assert_eq!(params.get_auto_tagging(), Some(0.5));
        params = params.add_auto_tagging(Some(1.5));
        assert_eq!(params.get_auto_tagging(), Some(1.0));
        params = params.add_auto_tagging(None);
        assert_eq!(params.get_auto_tagging(), None);
    }
}
