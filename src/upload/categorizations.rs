use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Categorizations {
    Google,
    GoogleVideo,
    Imagga,
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
