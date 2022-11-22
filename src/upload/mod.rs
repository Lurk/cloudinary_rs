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

use paste::paste;
use std::collections::{BTreeMap, HashMap, HashSet};

use self::{
    access_mode::AccessModes, allowed_headers::AllowedHeaders,
    background_removal::BackgroundRemoval, categorizations::Categorizations,
    data_types::Coordinates, data_types::DataType, delivery_type::DeliveryType,
    raw_convert::RawConvert, resource_type::ResourceTypes,
    responsive_breakpoints::ResponsiveBreakpoints,
};

#[derive(Debug)]
pub struct UploadOptions<'entry_key_lifetime> {
    inner: BTreeMap<&'entry_key_lifetime str, DataType>,
}

impl UploadOptions<'_> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
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

    pub fn add_context(mut self, key: String, value: String) -> Self {
        if let Some(DataType::HashMap(context)) = self.inner.get_mut("context") {
            context.insert(key, value);
        } else {
            self.inner
                .insert("context", DataType::HashMap(HashMap::from([(key, value)])));
        }

        self
    }

    pub fn remove_context(mut self, key: &str) -> Self {
        if let Some(DataType::HashMap(context)) = self.inner.get_mut("context") {
            context.remove(key);
            if context.is_empty() {
                self.inner.remove("context");
            }
        }
        self
    }

    pub fn get_context(&self, key: &String) -> Option<String> {
        if let Some(DataType::HashMap(context)) = self.inner.get("context") {
            return context.get(key).cloned();
        }
        None
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        if let Some(DataType::HashMap(metadata)) = self.inner.get_mut("metadata") {
            metadata.insert(key, value);
        } else {
            self.inner
                .insert("metadata", DataType::HashMap(HashMap::from([(key, value)])));
        }

        self
    }

    pub fn remove_metadata(mut self, key: &str) -> Self {
        if let Some(DataType::HashMap(metadata)) = self.inner.get_mut("metadata") {
            metadata.remove(key);
            if metadata.is_empty() {
                self.inner.remove("metadata");
            }
        }
        self
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
}

impl Default for UploadOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! add_field {
    ($struct_name:ident, $field:expr, $type:ty, $data_type:expr) => {
        paste! {
            impl $struct_name<'_> {
                pub fn [<set_ $field>](mut self,  value: $type )->Self{
                    self.inner.insert($field, $data_type(value));
                    self
                }

                pub fn [<get_ $field>](self)->Option<$type>{
                    if let Some($data_type(value)) = self.inner.get($field) {
                        return Some(value.clone());
                    }
                    None
                }
            }
        }
    };
}

add_field!(UploadOptions, "folder", String, DataType::String);
add_field!(UploadOptions, "upload_preset", String, DataType::String);
add_field!(UploadOptions, "type", DeliveryType, DataType::DeliveryType);
add_field!(
    UploadOptions,
    "access_mode",
    AccessModes,
    DataType::AccessModes
);
add_field!(UploadOptions, "public_id", String, DataType::String);
add_field!(UploadOptions, "use_filename", bool, DataType::Boolean);
add_field!(UploadOptions, "filename_override", String, DataType::String);
add_field!(
    UploadOptions,
    "resource_type",
    ResourceTypes,
    DataType::ResourceTypes
);
add_field!(
    UploadOptions,
    "discard_original_filename",
    bool,
    DataType::Boolean
);
add_field!(UploadOptions, "overwrite", bool, DataType::Boolean);
add_field!(UploadOptions, "colors", bool, DataType::Boolean);
add_field!(UploadOptions, "faces", bool, DataType::Boolean);
add_field!(UploadOptions, "quality_analysis", bool, DataType::Boolean);
add_field!(
    UploadOptions,
    "accessibility_analysis",
    bool,
    DataType::Boolean
);
add_field!(
    UploadOptions,
    "cinemagraph_analysis",
    bool,
    DataType::Boolean
);
add_field!(UploadOptions, "image_metadata", bool, DataType::Boolean);
add_field!(UploadOptions, "phash", bool, DataType::Boolean);
add_field!(
    UploadOptions,
    "responsive_breakpoints",
    Vec<ResponsiveBreakpoints>,
    DataType::ResponsiveBreakpoints
);
add_field!(
    UploadOptions,
    "categorization",
    Vec<Categorizations>,
    DataType::Categorization
);
add_field!(UploadOptions, "detection", String, DataType::String);
add_field!(UploadOptions, "ocr", String, DataType::String);
add_field!(UploadOptions, "eager", String, DataType::String);
add_field!(UploadOptions, "eager_async", bool, DataType::Boolean);
add_field!(
    UploadOptions,
    "eager_notification_url",
    String,
    DataType::String
);
add_field!(UploadOptions, "transformation", String, DataType::String);
add_field!(UploadOptions, "format", String, DataType::String);
add_field!(
    UploadOptions,
    "custom_coordinates",
    Coordinates,
    DataType::Coordinates
);
add_field!(
    UploadOptions,
    "face_coordinates",
    Vec<Coordinates>,
    DataType::FaceCoordinates
);
add_field!(
    UploadOptions,
    "background_removal",
    BackgroundRemoval,
    DataType::BackgroundRemoval
);
add_field!(
    UploadOptions,
    "raw_convert",
    RawConvert,
    DataType::RawConvert
);
add_field!(
    UploadOptions,
    "allowed_formats",
    Vec<String>,
    DataType::VecOfString
);
add_field!(UploadOptions, "async", bool, DataType::Boolean);
add_field!(UploadOptions, "backup", bool, DataType::Boolean);
add_field!(UploadOptions, "callback", String, DataType::String);
add_field!(UploadOptions, "eval", String, DataType::String);
add_field!(UploadOptions, "headers", HashMap<AllowedHeaders, String>, DataType::AllowedHeaders);
add_field!(UploadOptions, "invalidate", bool, DataType::Boolean);
add_field!(UploadOptions, "notification_url", String, DataType::String);
add_field!(UploadOptions, "proxy", String, DataType::String);
add_field!(
    UploadOptions,
    "return_delete_token",
    bool,
    DataType::Boolean
);

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
