use std::fmt::{Display, Formatter};

use super::aspect_ratio::AspectRatio;

#[derive(Debug, Clone)]
pub enum ResizeMode {
    /// Resizes the image to the specified width and aspect ratio.
    ScaleByWidth {
        width: u32,
        /// Aspect ratio - if not specified the original aspect ratio is preserved
        ar: Option<AspectRatio>,
        /// liquid - enables content-aware liquid rescaling (also sometimes known as 'seam carving'), which can be
        /// useful when changing the aspect ratio of an image.
        liquid: Option<()>,
    },
    /// Resizes the image to the specified height and aspect ratio.
    ScaleByHeight {
        height: u32,
        /// Aspect ratio - if not specified the original aspect ratio is preserved
        ar: Option<AspectRatio>,
        /// g_liquid - enables content-aware liquid rescaling (also sometimes known as 'seam carving'), which can be
        /// useful when changing the aspect ratio of an image.
        liquid: Option<()>,
    },
    /// Resizes the image to the specified dimensions without retaining the original aspect ratio.
    Scale {
        width: u32,
        height: u32,
        /// liquid - enables content-aware liquid rescaling (also sometimes known as 'seam carving'), which can be useful
        /// when changing the aspect ratio of an image.
        liquid: Option<()>,
    },
}

impl Display for ResizeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResizeMode::ScaleByWidth { ar, width, liquid } => write!(
                f,
                "{}c_scale,w_{}{}",
                ar.as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                width,
                liquid.map(|_| ",g_liquid").unwrap_or("")
            ),
            ResizeMode::ScaleByHeight { height, ar, liquid } => write!(
                f,
                "{}c_scale,h_{}{}",
                ar.as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                height,
                liquid.map(|_| ",g_liquid").unwrap_or("")
            ),
            ResizeMode::Scale {
                width,
                height,
                liquid,
            } => write!(
                f,
                "c_scale,w_{},h_{}{}",
                width,
                height,
                liquid.map(|_| ",g_liquid").unwrap_or("")
            ),
        }
    }
}
