use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use regex::Regex;
use reqwest::Url;

pub enum ResizeMode {
    /// Resizes the image to the specified width and aspect ratio.
    /// Optional parameters:
    /// * Aspect ratio - if not specified the original aspect ratio is preserved
    /// * g_liquid - enables content-aware liquid rescaling (also sometimes known as 'seam carving'),
    /// which can be useful when changing the aspect ratio of an image.
    ScaleByWidth(u32, Option<AspectRatio>, Option<()>),
    /// Resizes the image to the specified height and aspect ratio. If aspect ratio is not specified, it is preserved.
    ScaleByHeight(u32, Option<AspectRatio>, Option<()>),
    /// Resizes the image to the specified dimensions without retaining the original aspect ratio.
    Scale(u32, u32, Option<()>),
}

impl Display for ResizeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResizeMode::ScaleByWidth(width, aspect_ratio, g_liquid) => write!(
                f,
                "{}c_scale,w_{}{}",
                aspect_ratio
                    .as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                width,
                g_liquid.map(|_| ",g_liquid").unwrap_or("")
            ),
            ResizeMode::ScaleByHeight(height, aspect_ratio, g_liquid) => write!(
                f,
                "{}c_scale,h_{}{}",
                aspect_ratio
                    .as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                height,
                g_liquid.map(|_| ",g_liquid").unwrap_or("")
            ),
            ResizeMode::Scale(width, height, g_liquid) => write!(
                f,
                "c_scale,w_{},h_{}{}",
                width,
                height,
                g_liquid.map(|_| ",g_liquid").unwrap_or("")
            ),
        }
    }
}

pub enum AspectRatio {
    /// Ignore the aspect ratio of the input and stretch to exactly the given width or height values.
    Ignore,
    /// The usual way to represent aspect ratio is by using a colon, e.g. 4:3.
    Sides(u32, u32),
    /// A decimal value representing the width divided by the height (e.g., 0.5).
    Result(f32),
}

impl Display for AspectRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AspectRatio::Sides(width, height) => write!(f, "ar_{}:{}", width, height),
            AspectRatio::Result(result) => write!(f, "ar_{}", result),
            AspectRatio::Ignore => write!(f, "fl_ignore_aspect_ratio"),
        }
    }
}

pub enum Transformations {
    /// These modes adjust the size of the delivered image without cropping out any elements of the original image.
    Resize(ResizeMode),
}

impl Display for Transformations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Transformations::Resize(resize_mode) => write!(f, "{}", resize_mode),
        }
    }
}

pub struct Image {
    cloud_name: Arc<str>,
    public_id: Arc<str>,
    format: Option<Arc<str>>,
    transformations: Vec<Transformations>,
}

impl Image {
    pub fn new(cloud_name: Arc<str>, public_id: Arc<str>) -> Self {
        Image {
            cloud_name,
            public_id,
            format: None,
            transformations: Vec::new(),
        }
    }

    pub fn set_format(&mut self, format: &str) {
        self.format = Some(format.into());
    }

    pub fn get_format(&self) -> Option<Arc<str>> {
        self.format.clone()
    }

    pub fn add_transformation(&mut self, transformation: Transformations) {
        self.transformations.push(transformation);
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
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::Scale(100, 100, None)));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/c_scale,w_100,h_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_width() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::ScaleByWidth(
            100, None, None,
        )));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/c_scale,w_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_height() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::ScaleByHeight(
            100, None, None,
        )));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/c_scale,h_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_width_with_aspect_ratio() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::ScaleByWidth(
            100,
            Some(AspectRatio::Sides(16, 9)),
            None,
        )));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/ar_16:9,c_scale,w_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_height_with_aspect_ratio() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::ScaleByHeight(
            100,
            Some(AspectRatio::Result(0.5)),
            None,
        )));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/ar_0.5,c_scale,h_100/path/name"
        );
    }

    #[test]
    fn add_scale_by_width_with_aspect_ratio_and_liquid() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::ScaleByWidth(
            100,
            Some(AspectRatio::Sides(16, 9)),
            Some(()),
        )));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/ar_16:9,c_scale,w_100,g_liquid/path/name"
        );
    }

    #[test]
    fn scale_ignore_aspect_ratio() {
        let mut image = Image::new("test".into(), "path/name".into());
        image.add_transformation(Transformations::Resize(ResizeMode::ScaleByWidth(
            100,
            Some(AspectRatio::Ignore),
            None,
        )));
        assert_eq!(
            image.build().as_str(),
            "https://res.cloudinary.com/test/image/upload/fl_ignore_aspect_ratio,c_scale,w_100/path/name"
        );
    }
}
