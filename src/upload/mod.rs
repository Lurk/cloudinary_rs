mod access_mode;
mod allowed_headers;
mod background_removal;
mod categorizations;
mod delivery_type;
mod raw_convert;
mod resource_type;
mod responsive_breakpoints;

use itertools::Itertools;
use reqwest::Url;
use std::collections::{BTreeMap, HashMap, HashSet};

use self::{
    access_mode::AccessModes, allowed_headers::AllowedHeaders,
    background_removal::BackgroundRemoval, categorizations::Categorizations,
    delivery_type::DeliveryType, raw_convert::RawConvert, resource_type::ResourceTypes,
    responsive_breakpoints::ResponsiveBreakpoints,
};

pub type Coordinates = [u32; 4];

pub struct UploadOptions {
    pub upload_preset: Option<String>,
    pub r#type: Option<DeliveryType>,
    pub access_mode: Option<AccessModes>,
    pub public_id: Option<String>,
    pub folder: Option<String>,
    pub use_filename: Option<bool>,
    pub filename_override: Option<String>,
    pub resource_type: Option<ResourceTypes>,
    pub discard_original_filename: Option<bool>,
    pub overwrite: Option<bool>,
    tags: Option<HashSet<String>>,
    context: Option<HashMap<String, String>>,
    metadata: Option<HashMap<String, String>>,
    pub colors: Option<bool>,
    pub faces: Option<bool>,
    pub quality_analysis: Option<bool>,
    pub accessibility_analysis: Option<bool>,
    pub cinemagraph_analysis: Option<bool>,
    pub image_metadata: Option<bool>,
    pub phash: Option<bool>,
    pub responsive_breakpoints: Option<Vec<ResponsiveBreakpoints>>,
    auto_tagging: Option<f32>,
    pub categorization: Option<Vec<Categorizations>>,
    pub detection: Option<String>,
    pub ocr: Option<String>,
    pub eager: Option<String>,
    pub eager_async: Option<bool>,
    pub eager_notification_url: Option<String>,
    pub transformation: Option<String>,
    pub format: Option<String>,
    pub custom_coordinates: Option<Coordinates>,
    pub face_coordinates: Option<Vec<Coordinates>>,
    pub background_removal: Option<BackgroundRemoval>,
    pub raw_convert: Option<RawConvert>,
    pub allowed_formats: Option<Vec<String>>,
    pub r#async: Option<bool>,
    pub backup: Option<bool>,
    pub callback: Option<String>,
    pub eval: Option<String>,
    pub headers: Option<HashMap<AllowedHeaders, String>>,
    pub invalidate: Option<bool>,
    pub moderation: Option<String>,
    pub notification_url: Option<Url>,
    pub proxy: Option<Url>,
    pub return_delete_token: Option<bool>,
}

impl UploadOptions {
    pub fn new() -> Self {
        UploadOptions {
            upload_preset: None,
            r#type: None,
            access_mode: None,
            public_id: None,
            folder: None,
            use_filename: None,
            filename_override: None,
            resource_type: None,
            discard_original_filename: None,
            overwrite: None,
            tags: None,
            context: None,
            metadata: None,
            colors: None,
            faces: None,
            quality_analysis: None,
            accessibility_analysis: None,
            cinemagraph_analysis: None,
            image_metadata: None,
            phash: None,
            responsive_breakpoints: None,
            auto_tagging: None,
            categorization: None,
            detection: None,
            ocr: None,
            eager: None,
            eager_async: None,
            eager_notification_url: None,
            transformation: None,
            format: None,
            custom_coordinates: None,
            face_coordinates: None,
            background_removal: None,
            raw_convert: None,
            allowed_formats: None,
            r#async: None,
            backup: None,
            callback: None,
            eval: None,
            headers: None,
            invalidate: None,
            moderation: None,
            notification_url: None,
            proxy: None,
            return_delete_token: None,
        }
    }

    pub fn to_map(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        if let Some(upload_preset) = &self.upload_preset {
            map.insert("upload_preset".to_string(), upload_preset.clone());
        }
        if let Some(delivery_type) = &self.r#type {
            map.insert("type".to_string(), delivery_type.to_string());
        }
        if let Some(access_mode) = &self.access_mode {
            map.insert("access_mode".to_string(), access_mode.to_string());
        }
        if let Some(public_id) = &self.public_id {
            map.insert("public_id".to_string(), public_id.clone());
        }
        if let Some(folder) = &self.folder {
            map.insert("folder".to_string(), folder.clone());
        }
        if let Some(use_filename) = &self.use_filename {
            map.insert("use_filename".to_string(), use_filename.to_string());
        }
        if let Some(filename_override) = &self.filename_override {
            map.insert("filename_override".to_string(), filename_override.clone());
        }
        if let Some(resource_type) = &self.resource_type {
            map.insert("resource_type".to_string(), resource_type.to_string());
        }
        if let Some(discard_original_filename) = &self.discard_original_filename {
            map.insert(
                "discard_original_filename".to_string(),
                discard_original_filename.to_string(),
            );
        }
        if let Some(overwrite) = &self.overwrite {
            map.insert("overwrite".to_string(), overwrite.to_string());
        }
        if let Some(tags) = &self.tags {
            let joined_tags: String = tags.iter().join("', '");
            map.insert("tags".to_string(), format!("['{}']", joined_tags));
        }
        if let Some(context) = &self.context {
            let joined_context: String = context.iter().map(|(k, v)| format!("{k}={v}")).join("|");
            map.insert("context".to_string(), joined_context);
        }
        if let Some(metadata) = &self.metadata {
            let joined_metadata: String =
                metadata.iter().map(|(k, v)| format!("{k}={v}")).join("|");
            map.insert("metadata".to_string(), joined_metadata);
        }
        if let Some(colors) = &self.colors {
            map.insert("colors".to_string(), colors.to_string());
        }
        if let Some(faces) = &self.faces {
            map.insert("faces".to_string(), faces.to_string());
        }
        if let Some(quality_analysis) = &self.quality_analysis {
            map.insert("quality_analysis".to_string(), quality_analysis.to_string());
        }
        if let Some(accessibility_analysis) = &self.accessibility_analysis {
            map.insert(
                "accessibility_analysis".to_string(),
                accessibility_analysis.to_string(),
            );
        }
        if let Some(cinemagraph_analysis) = &self.cinemagraph_analysis {
            map.insert(
                "cinemagraph_analysis".to_string(),
                cinemagraph_analysis.to_string(),
            );
        }
        if let Some(image_metadata) = &self.image_metadata {
            map.insert("image_metadata".to_string(), image_metadata.to_string());
        }
        if let Some(phash) = &self.phash {
            map.insert("phash".to_string(), phash.to_string());
        }
        if let Some(responsive_breakpoints) = &self.responsive_breakpoints {
            let joined_breakpoints = responsive_breakpoints
                .iter()
                .map(|breakpoint| serde_json::to_string(breakpoint).unwrap())
                .join("', '");
            map.insert(
                "responsive_breakpoints".to_string(),
                format!("['{}']", joined_breakpoints),
            );
        }
        if let Some(auto_tagging) = &self.auto_tagging {
            map.insert("auto_tagging".to_string(), auto_tagging.to_string());
        }
        if let Some(categorizations) = &self.categorization {
            map.insert(
                "categorization".to_string(),
                categorizations.iter().join(","),
            );
        }
        if let Some(detection) = &self.detection {
            map.insert("detection".to_string(), detection.clone());
        }
        if let Some(ocr) = &self.ocr {
            map.insert("ocr".to_string(), ocr.clone());
        }
        if let Some(eager) = &self.eager {
            map.insert("eager".to_string(), eager.clone());
        }
        if let Some(eager_async) = &self.eager_async {
            map.insert("eager_async".to_string(), eager_async.to_string());
        }
        if let Some(eager_notification_url) = &self.eager_notification_url {
            map.insert(
                "eager_notification_url".to_string(),
                eager_notification_url.clone(),
            );
        }
        if let Some(transformation) = &self.transformation {
            map.insert("transformation".to_string(), transformation.clone());
        }
        if let Some(format) = &self.format {
            map.insert("format".to_string(), format.clone());
        }
        if let Some(custom_coordinates) = &self.custom_coordinates {
            map.insert(
                "custom_coordinates".to_string(),
                custom_coordinates.iter().join(","),
            );
        }
        if let Some(face_coordinates) = &self.face_coordinates {
            map.insert(
                "face_coordinates".to_string(),
                face_coordinates
                    .iter()
                    .map(|c| c.iter().join(","))
                    .join("|"),
            );
        }
        if let Some(background_removal) = &self.background_removal {
            map.insert(
                "background_removal".to_string(),
                background_removal.to_string(),
            );
        }
        if let Some(raw_convert) = &self.raw_convert {
            map.insert("raw_convert".to_string(), raw_convert.to_string());
        }
        if let Some(allowed_formats) = &self.allowed_formats {
            map.insert(
                "allowed_formats".to_string(),
                allowed_formats.iter().join(","),
            );
        }
        if let Some(r#async) = &self.r#async {
            map.insert("async".to_string(), r#async.to_string());
        }
        if let Some(backup) = &self.backup {
            map.insert("backup".to_string(), backup.to_string());
        }
        if let Some(callback) = &self.callback {
            map.insert("callback".to_string(), callback.clone());
        }
        if let Some(eval) = &self.eval {
            map.insert("eval".to_string(), eval.clone());
        }
        if let Some(headers) = &self.headers {
            let joined_headers: String =
                headers.iter().map(|(k, v)| format!("{k}: {v}")).join("\n");
            map.insert("headers".to_string(), joined_headers);
        }
        if let Some(invalidate) = &self.invalidate {
            map.insert("invalidate".to_string(), invalidate.to_string());
        }
        if let Some(moderation) = &self.moderation {
            map.insert("moderation".to_string(), moderation.to_string());
        }
        if let Some(notification_url) = &self.notification_url {
            map.insert("notification_url".to_string(), notification_url.to_string());
        }
        if let Some(proxy) = &self.proxy {
            map.insert("proxy".to_string(), proxy.to_string());
        }
        if let Some(return_delete_token) = &self.return_delete_token {
            map.insert(
                "return_delete_token".to_string(),
                return_delete_token.to_string(),
            );
        }

        map
    }

    pub fn set_auto_tagging(mut self, auto_tagging: Option<f32>) -> Self {
        if let Some(value) = auto_tagging {
            self.auto_tagging = Some(value.clamp(0.0, 1.0));
        }
        self.auto_tagging = None;
        self
    }

    pub fn auto_tagging(&self) -> Option<f32> {
        self.auto_tagging
    }

    pub fn tags(&self) -> Option<&HashSet<String>> {
        self.tags.as_ref()
    }

    fn add_tags(mut self, tags: &[String]) -> Self {
        match self.tags {
            Some(mut inner_tags) => {
                inner_tags.extend(tags.iter().cloned());
                self.tags = Some(inner_tags);
            }
            None => {
                self.tags = Some(HashSet::from_iter(tags.iter().cloned()));
            }
        }
        self
    }

    fn remove_tags(mut self, tags: &[String]) -> Self {
        if let Some(mut inner_tags) = self.tags {
            for tag in tags {
                inner_tags.remove(tag);
            }
            if inner_tags.is_empty() {
                self.tags = None;
            } else {
                self.tags = Some(inner_tags);
            }
        }
        self
    }

    pub fn add_context(mut self, key: String, value: String) -> Self {
        match self.context {
            Some(mut context) => {
                context.insert(key, value);
                self.context = Some(context);
            }
            None => {
                self.context = Some(HashMap::from([(key, value)]));
            }
        }
        self
    }
    pub fn remove_context(mut self, key: &str) -> Self {
        if let Some(mut context) = self.context {
            context.remove(key);
            if context.is_empty() {
                self.context = None;
            } else {
                self.context = Some(context);
            }
        }
        self
    }
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        match self.metadata {
            Some(mut metadata) => {
                metadata.insert(key, value);
                self.metadata = Some(metadata);
            }
            None => {
                self.metadata = Some(HashMap::from([(key, value)]));
            }
        }
        self
    }
    pub fn remove_metadata(mut self, key: &str) -> Self {
        if let Some(mut metadata) = self.metadata {
            metadata.remove(key);
            if metadata.is_empty() {
                self.metadata = None;
            } else {
                self.metadata = Some(metadata);
            }
        }
        self
    }
}

impl Default for UploadOptions {
    fn default() -> Self {
        Self::new()
    }
}
