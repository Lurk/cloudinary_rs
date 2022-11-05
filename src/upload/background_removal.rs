use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundRemoval {
    CloudinaryAi,
    Pixelz,
}

impl fmt::Display for BackgroundRemoval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BackgroundRemoval::CloudinaryAi => write!(f, "cloudinary_ai"),
            BackgroundRemoval::Pixelz => write!(f, "pixelz"),
        }
    }
}
