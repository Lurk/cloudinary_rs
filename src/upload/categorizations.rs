use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Categorizations {
    /// [Google Automatic Video Tagging](https://cloudinary.com/documentation/google_automatic_video_tagging_addon#adding_resource_tags_to_videos)
    Google,
    /// [Google Auto Tagging](https://cloudinary.com/documentation/google_auto_tagging_addon#adding_resource_tags_to_images)
    GoogleVideo,
    /// [Imagga Auto Tagging](https://cloudinary.com/documentation/imagga_auto_tagging_addon#adding_resource_tags_to_images)
    Imagga,
    /// [Amazon Rekognition Auto Tagging](https://cloudinary.com/documentation/aws_rekognition_auto_tagging_addon#automatically_adding_tags_to_images)
    AwsRek,
}

impl fmt::Display for Categorizations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Categorizations::Google => write!(f, "google_tagging"),
            Categorizations::GoogleVideo => write!(f, "google_video_tagging"),
            Categorizations::Imagga => write!(f, "imagga_tagging"),
            Categorizations::AwsRek => write!(f, "auto_tagging"),
        }
    }
}
