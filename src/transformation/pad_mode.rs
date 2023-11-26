use std::fmt::Display;

use super::{aspect_ratio::AspectRatio, background::Background, gravity::Gravity};

/// Resizes the asset to fill the specified width and height while retaining the original aspect ratio
/// (by default) and with all of the original asset visible. If the proportions of the original asset do not match
/// the specified width and height, padding is added to the asset to reach the required size. You can also specify
/// where the original asset is placed using the gravity parameter (set to center by default). Additionally, you
/// can specify the color of the background in the case that padding is added.
#[derive(Debug, Clone)]
pub enum PadMode {
    PadByWidth {
        width: u32,
        ar: Option<AspectRatio>,
        background: Option<Background>,
        gravity: Option<Gravity>,
    },
    PadByHeight {
        height: u32,
        ar: Option<AspectRatio>,
        background: Option<Background>,
        gravity: Option<Gravity>,
    },
    Pad {
        width: u32,
        height: u32,
        background: Option<Background>,
        gravity: Option<Gravity>,
    },
}

impl Display for PadMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PadMode::PadByWidth {
                width,
                ar,
                background,
                gravity,
            } => write!(
                f,
                "{}{}c_pad{},w_{}",
                background
                    .as_ref()
                    .map(|b| format!("{},", b))
                    .unwrap_or("".into()),
                ar.as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                gravity
                    .as_ref()
                    .map(|g| format!(",{}", g))
                    .unwrap_or("".into()),
                width,
            ),
            PadMode::PadByHeight {
                height,
                ar,
                background,
                gravity,
            } => write!(
                f,
                "{}{}c_pad{},h_{}",
                background
                    .as_ref()
                    .map(|b| format!("{},", b))
                    .unwrap_or("".into()),
                ar.as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                gravity
                    .as_ref()
                    .map(|g| format!(",{}", g))
                    .unwrap_or("".into()),
                height,
            ),
            PadMode::Pad {
                width,
                height,
                background,
                gravity,
            } => write!(
                f,
                "{}c_pad{},w_{},h_{}",
                background
                    .as_ref()
                    .map(|b| format!("{},", b))
                    .unwrap_or("".into()),
                gravity
                    .as_ref()
                    .map(|g| format!(",{}", g))
                    .unwrap_or("".into()),
                width,
                height,
            ),
        }
    }
}
