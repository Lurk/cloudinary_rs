use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;

#[derive(Debug)]
pub enum Moderation {
    /// `manual` to add the uploaded asset to a list of pending assets that can be moderated using the Admin API or the
    /// [Cloudinary Console](https://console.cloudinary.com/console/media_library).
    ///
    /// For all asset types.
    Manual,
    /// `perception_point` to automatically moderate the uploaded asset using the
    /// [Perception Point Malware Detection add-on](https://cloudinary.com/documentation/perception_point_malware_detection_addon).
    ///
    /// For all asset types.
    PerceptionPoint,
    /// `webpurify` to automatically moderate the uploaded image using the
    /// [WebPurify Image Moderation add-on](https://cloudinary.com/documentation/webpurify_image_moderation_addon).
    ///
    /// For images only.
    Webpurify,
    /// `aws_rek` to automatically moderate the uploaded image using the
    /// [Amazon Rekognition AI Moderation add-on](https://cloudinary.com/documentation/aws_rekognition_ai_moderation_addon).
    ///
    /// For images only.
    AwsRek,
    /// `duplicate:<threshold>` to detect if the same or a similar image already exists using the
    /// [Cloudinary Duplicate Image Detection add-on](https://cloudinary.com/documentation/cloudinary_duplicate_image_detection_addon).
    /// Set threshold to a float greater than 0 and less than or equal to 1.0 to specify how similar an image needs
    /// to be in order to be considered a duplicate. Set threshold to 0 to add an image to the index of images that
    /// are searched when duplicate detection is invoked for another image.
    ///
    /// For images only.
    Duplicate(f32),
    /// `aws_rek_video` to automatically moderate the uploaded video using the
    /// [Amazon Rekognition Video Moderation add-on](https://cloudinary.com/documentation/aws_rekognition_video_moderation_addon).
    ///
    /// For videos only.
    AwsRekVideo,
    /// `google_video_moderation` automatically moderate the uploaded video using the
    /// [Google AI Video Moderation add-on](https://cloudinary.com/documentation/google_ai_video_moderation_addon).
    ///
    /// For videos only.
    GoogleVideoModeration,
}

impl Display for Moderation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Moderation::Manual => f.write_str("manual"),
            Moderation::PerceptionPoint => f.write_str("perception_point"),
            Moderation::Webpurify => f.write_str("webpurify"),
            Moderation::AwsRek => f.write_str("aws_rek"),
            Moderation::Duplicate(threshold) => write!(f, "duplicate:{}", threshold),
            Moderation::AwsRekVideo => f.write_str("aws_rek_video"),
            Moderation::GoogleVideoModeration => f.write_str("google_video_moderation"),
        }
    }
}

impl PartialEq for Moderation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Duplicate(_), Self::Duplicate(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Moderation {}

impl Hash for Moderation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Moderation::Manual => "manual".hash(state),
            Moderation::PerceptionPoint => "perception_point".hash(state),
            Moderation::Webpurify => "webpurify".hash(state),
            Moderation::AwsRek => "aws_rek".hash(state),
            Moderation::Duplicate(_) => "duplicate".hash(state),
            Moderation::AwsRekVideo => "aws_rek_video".hash(state),
            Moderation::GoogleVideoModeration => "google_video_moderation".hash(state),
        }
    }
}
