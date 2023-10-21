pub mod aspect_ratio;
pub mod crop_mode;
pub mod gravity;
pub mod resize_mode;

use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    sync::Arc,
};

use regex::Regex;
use url::Url;

use self::{crop_mode::CropMode, resize_mode::ResizeMode};

#[derive(Debug, Clone)]
pub enum Transformations {
    /// These modes adjust the size of the delivered image without cropping out any elements of the original image.
    Resize(ResizeMode),
    Crop(CropMode),
}

impl Display for Transformations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Transformations::Resize(resize_mode) => write!(f, "{}", resize_mode),
            Transformations::Crop(crop_mode) => write!(f, "{}", crop_mode),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    cloud_name: Arc<str>,
    public_id: Arc<str>,
    format: Option<Arc<str>>,
    transformations: RefCell<Vec<Transformations>>,
}

impl Image {
    pub fn new(cloud_name: Arc<str>, public_id: Arc<str>) -> Self {
        Image {
            cloud_name,
            public_id,
            format: None,
            transformations: RefCell::new(Vec::new()),
        }
    }

    pub fn set_format(&mut self, format: &str) {
        self.format = Some(format.into());
    }

    pub fn get_format(&self) -> Option<Arc<str>> {
        self.format.clone()
    }

    pub fn add_transformation(self, transformation: Transformations) -> Self {
        self.transformations.borrow_mut().push(transformation);
        self
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
        let transformations = self
            .transformations
            .borrow()
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("/");
        let path = format!(
            "{}/image/upload/{}{}",
            self.cloud_name,
            if transformations.is_empty() {
                "".into()
            } else {
                format!("{}/", transformations)
            },
            self.public_id
        );

        match self.get_format() {
            Some(format) => {
                let file_name = self.public_id.split('/').last().unwrap().to_string();

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

impl From<Image> for Url {
    fn from(image: Image) -> Self {
        image.build()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
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
    use crate::transformation::aspect_ratio::AspectRatio;

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
        .unwrap()
        .try_into()
        .unwrap();

        assert_eq!(image.cloud_name, "cloud".into());
        assert_eq!(image.public_id, "path/with%20space/name".into());
        assert_eq!(image.get_format(), Some("png".into()));
    }

    #[test]
    fn from_url_without_version() {
        let image: Image =
            Url::parse("https://res.cloudinary.com/test/image/upload/path/with%20space/name.png")
                .unwrap()
                .try_into()
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

    #[test]
    fn add_scale() {
        let image = Image::new("test".into(), "path/name".into()).add_transformation(
            Transformations::Resize(ResizeMode::Scale {
                width: 100,
                height: 100,
                liquid: None,
            }),
        );
        assert_eq!(
            image.to_string(),
            "https://res.cloudinary.com/test/image/upload/c_scale,w_100,h_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_width() {
        let image = Image::new("test".into(), "path/name".into()).add_transformation(
            Transformations::Resize(ResizeMode::ScaleByWidth {
                width: 100,
                ar: None,
                liquid: None,
            }),
        );
        assert_eq!(
            image.to_string(),
            "https://res.cloudinary.com/test/image/upload/c_scale,w_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_height() {
        let image_url: Url = Image::new("test".into(), "path/name".into())
            .add_transformation(Transformations::Resize(ResizeMode::ScaleByHeight {
                height: 100,
                ar: None,
                liquid: None,
            }))
            .into();
        assert_eq!(
            image_url.as_str(),
            "https://res.cloudinary.com/test/image/upload/c_scale,h_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_width_with_aspect_ratio() {
        let image = Image::new("test".into(), "path/name".into()).add_transformation(
            Transformations::Resize(ResizeMode::ScaleByWidth {
                width: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                liquid: None,
            }),
        );
        assert_eq!(
            image.to_string(),
            "https://res.cloudinary.com/test/image/upload/ar_16:9,c_scale,w_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_height_with_aspect_ratio() {
        let image = Image::new("test".into(), "path/name".into()).add_transformation(
            Transformations::Resize(ResizeMode::ScaleByHeight {
                height: 100,
                ar: Some(AspectRatio::Result(0.5)),
                liquid: None,
            }),
        );
        assert_eq!(
            image.to_string(),
            "https://res.cloudinary.com/test/image/upload/ar_0.5,c_scale,h_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_width_with_aspect_ratio_and_liquid() {
        let image_url: Url = Image::new("test".into(), "path/name".into())
            .add_transformation(Transformations::Resize(ResizeMode::ScaleByWidth {
                width: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                liquid: Some(()),
            }))
            .into();
        assert_eq!(
            image_url.as_str(),
            "https://res.cloudinary.com/test/image/upload/ar_16:9,c_scale,w_100,g_liquid/path/name"
        );
    }

    #[test]
    fn scale_ignore_aspect_ratio() {
        let image_url: Url = Image::new("test".into(), "path/name".into())
            .add_transformation(Transformations::Resize(ResizeMode::ScaleByWidth {
                width: 100,
                ar: Some(AspectRatio::Ignore),
                liquid: None,
            }))
            .into();
        assert_eq!(
            image_url.as_str(),
            "https://res.cloudinary.com/test/image/upload/fl_ignore_aspect_ratio,c_scale,w_100/path/name"
        );
    }
}

// https://res.cloudinary.com/barhamon/image/upload/c_fill,w_1190/tmb_preparations/thermarest_neoair_max.jpg
// https://res.cloudinary.com/barhamon/image/upload/c_fill,g_auto,w_1190/tmb_preparations/thermarest_neoair_max.jpg
// https://res.cloudinary.com/barhamon/image/upload/c_scale,w_1190/tmb_preparations/thermarest_neoair_max.jpg
// https://res.cloudinary.com/barhamon/image/upload/c_fill,h_1190/tmb_preparations/thermarest_neoair_max.jpg
