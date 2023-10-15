use std::{collections::HashMap, sync::Arc};

use regex::Regex;
use reqwest::Url;

pub struct Image {
    cloud_name: Arc<str>,
    public_id: Arc<str>,
    transformations: HashMap<Arc<str>, Arc<str>>,
}

impl Image {
    pub fn new(cloud_name: Arc<str>, public_id: Arc<str>) -> Self {
        Image {
            cloud_name,
            public_id,
            transformations: HashMap::new(),
        }
    }

    pub fn set_format(&mut self, format: &str) {
        self.transformations.insert("format".into(), format.into());
    }

    pub fn get_format(&self) -> Option<Arc<str>> {
        self.transformations.get("format").cloned()
    }

    /// Build a URL
    ///
    /// # Example:
    /// ```rust
    /// use cloudinary::transformation::Image;
    /// let mut image = Image::new("cloud_name".into(), "public_id".into());
    /// image.set_format("jpg");
    /// assert_eq!(image.build().as_str(), "https://res.cloudinary.com/cloud_name/image/upload/public_id.jpg");
    /// ```
    pub fn build(&self) -> Url {
        let mut url = Url::parse("https://res.cloudinary.com").unwrap();
        let path = format!("{}/image/upload/{}", self.cloud_name, self.public_id);

        let file_name = self.public_id.split('/').last().unwrap().to_string();

        match self.get_format() {
            Some(format) => {
                let new_flie_name = format!(
                    "{}.{}",
                    file_name.split('.').collect::<Vec<&str>>().pop().unwrap(),
                    format
                );
                url.set_path(
                    path.replace(file_name.as_str(), new_flie_name.as_str())
                        .as_str(),
                );
            }
            None => {
                url.set_path(path.as_str());
            }
        }

        url
    }
}

impl TryFrom<Url> for Image {
    type Error = &'static str;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let cloudinary_regex: Regex = Regex::new(r"^.+\.cloudinary\.com/(?:([^/]+)/)image/?(?:(upload|fetch|private|authenticated|sprite|facebook|twitter|youtube|vimeo)/)?(?:(?:[^_/]+_[^,/]+,?)*/)?(?:v(\d+|\w{1,2})/)?([^\.^\s]+)(?:\.(.+))?$").unwrap();

        let link = url.as_str();
        if link.is_empty() {
            return Err("Empty url");
        }

        println!("{:?}", cloudinary_regex.captures(link));

        let parts = cloudinary_regex.captures(link).map(|caps| {
            (
                caps.get(1).unwrap().as_str(),
                caps.get(4).unwrap().as_str(),
                caps.get(5).unwrap().as_str(),
            )
        });

        if let Some((cloud_name, public_id, extension)) = parts {
            let mut image = Image::new(cloud_name.into(), public_id.into());
            if !extension.is_empty() {
                image.set_format(extension);
            }
            Ok(image)
        } else {
            Err("Invalid url")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_url_with_transformations() {
        let image: Image = Url::parse("https://res.cloudinary.com/i/image/upload/c_scale,h_800,q_auto/v1233456678/path/name.jpg")
            .ok().unwrap().try_into().ok().unwrap();

        assert_eq!(image.cloud_name, "i".into());
        assert_eq!(image.public_id, "path/name".into());
        assert_eq!(image.get_format(), Some("jpg".into()));
    }

    #[test]
    fn from_url_without_transformations() {
        let image: Image = Url::parse(
            "https://res.cloudinary.com/cloud/image/upload/v1233456678/path/with%20space/name.png",
        )
        .ok()
        .unwrap()
        .try_into()
        .ok()
        .unwrap();

        assert_eq!(image.cloud_name, "cloud".into());
        assert_eq!(image.public_id, "path/with%20space/name".into());
        assert_eq!(image.get_format(), Some("png".into()));
    }

    #[test]
    fn from_url_without_version() {
        let image: Image =
            Url::parse("https://res.cloudinary.com/test/image/upload/path/with%20space/name.png")
                .ok()
                .unwrap()
                .try_into()
                .ok()
                .unwrap();

        assert_eq!(image.cloud_name, "test".into());
        assert_eq!(image.public_id, "path/with%20space/name".into());
        assert_eq!(image.get_format(), Some("png".into()));
    }

    #[test]
    fn set_format() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.set_format("png");
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/path/name.png"
        );
    }

    #[test]
    fn replace_format() {
        let mut image: Image = Url::parse(
            "https://res.cloudinary.com/i/image/upload/c_scale,h_800,q_auto/path/name.jpg",
        )
        .unwrap()
        .try_into()
        .unwrap();

        image.set_format("png");
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/i/image/upload/path/name.png"
        );
    }
}
