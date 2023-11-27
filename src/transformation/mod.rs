pub mod aspect_ratio;
pub mod background;
pub mod crop_mode;
pub mod gravity;
pub mod named_color;
pub mod pad_mode;
pub mod resize_mode;

use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    sync::Arc,
};

use url::Url;

use self::{crop_mode::CropMode, pad_mode::PadMode, resize_mode::ResizeMode};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Transformations {
    /// These modes adjust the size of the delivered image without cropping out any elements of the original image.
    Resize(ResizeMode),
    Crop(CropMode),
    Pad(PadMode),
}

impl Display for Transformations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Transformations::Resize(resize_mode) => write!(f, "{}", resize_mode),
            Transformations::Crop(crop_mode) => write!(f, "{}", crop_mode),
            Transformations::Pad(pad_mode) => write!(f, "{}", pad_mode),
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

/// Check if the string is a transformation
/// A transformation is a string that has underscore and the length of the head is less than 4
/// https://support.cloudinary.com/hc/en-us/community/posts/4414437232018-Invalid-transformation-parameter
fn is_transformation(s: &str) -> bool {
    if let Some((head, _)) = s.split_once('_') {
        return head.len() < 4;
    }
    false
}

/// Check if the string is a version where version is a string that starts with 'v' and the rest of the string a Unix
/// timestamp
/// https://support.cloudinary.com/hc/en-us/articles/202520912-What-are-image-versions-
fn is_version(s: &str) -> bool {
    s.starts_with('v') && s.len() == 11 && s[1..].chars().all(|c| c.is_ascii_digit())
}

/// Parse a URL to an Image
/// Unofficial. Can break at any time.
/// Official recommendation is to use public_id that you get after uploading an image to Cloudinary.
impl TryFrom<Url> for Image {
    type Error = &'static str;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        if url.host_str().unwrap() != "res.cloudinary.com" {
            return Err("Not a cloudinary url");
        }

        let mut cloud_name: Option<&str> = None;
        let mut public_id_parts: Vec<(&str, Option<&str>)> = Vec::new();
        let mut public_id_teritory = false;
        for (pos, s) in url.path_segments().unwrap().enumerate() {
            match pos {
                0 => {
                    cloud_name = Some(s);
                }
                1 => {
                    if s != "image" {
                        return Err("Only image is supported");
                    }
                }
                2 => {
                    if ![
                        "upload",
                        "fetch",
                        "private",
                        "authenticated",
                        "sprite",
                        "facebook",
                        "twitter",
                        "youtube",
                        "vimeo",
                    ]
                    .contains(&s)
                    {
                        return Err("Invalid mode");
                    }
                }
                _ => {
                    if !public_id_teritory && is_version(s) {
                        public_id_teritory = true;
                    } else if !public_id_teritory && is_transformation(s) {
                    } else if let Some((head, tail)) = s.rsplit_once('.') {
                        public_id_teritory = true;
                        public_id_parts.push((head, Some(tail)));
                    } else {
                        public_id_teritory = true;
                        public_id_parts.push((s, None));
                    }
                }
            }
        }

        let cloud_name = cloud_name.ok_or("No cloud_name is found")?;
        let last = public_id_parts.pop().ok_or("no public_id is found")?;
        let mut public_id = public_id_parts
            .iter()
            .map(|(head, tail)| {
                if let Some(tail) = tail {
                    format!("{}.{}/", head, tail)
                } else {
                    format!("{}/", head)
                }
            })
            .collect::<Vec<String>>()
            .join("");

        public_id.push_str(last.0);
        let format = last.1;

        let mut image = Image::new(cloud_name.into(), public_id.into());
        if let Some(extension) = format {
            image.set_format(extension);
        }

        Ok(image)
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

    #[test]
    fn from_url_with_dots_in_transformation() {
        let image: Image =
            Url::parse("https://res.cloudinary.com/test/image/upload/ar_0.5,foo/1.jpg")
                .unwrap()
                .try_into()
                .unwrap();

        assert_eq!(image.cloud_name, "test".into());
        assert_eq!(image.public_id, "1".into());
        assert_eq!(image.get_format(), Some("jpg".into()));
    }

    #[test]
    fn from_url_with_dots_in_public_id() {
        let image: Image =
            Url::parse("https://res.cloudinary.com/test/image/upload/ar_0.5,foo/1.2.jpg")
                .unwrap()
                .try_into()
                .unwrap();

        assert_eq!(image.cloud_name, "test".into());
        assert_eq!(image.public_id, "1.2".into());
        assert_eq!(image.get_format(), Some("jpg".into()));
    }

    #[test]
    fn from_url_without_extension() {
        let image: Image =
            Url::parse("https://res.cloudinary.com/test/image/upload/ar_0.5,foo/v1640995200/1")
                .unwrap()
                .try_into()
                .unwrap();

        assert_eq!(image.cloud_name, "test".into());
        assert_eq!(image.public_id, "1".into());
        assert_eq!(image.get_format(), None);
    }

    #[test]
    fn pad_mode() {
        let image_url: Url = Image::new("test".into(), "path/name".into())
            .add_transformation(Transformations::Pad(PadMode::PadByWidth {
                width: 100,
                ar: None,
                background: None,
                gravity: None,
            }))
            .into();
        assert_eq!(
            image_url.as_str(),
            "https://res.cloudinary.com/test/image/upload/c_pad,w_100/path/name"
        );
    }
}
