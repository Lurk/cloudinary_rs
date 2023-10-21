use std::fmt::Display;

use super::{aspect_ratio::AspectRatio, gravity::Gravity};

#[derive(Debug, Clone)]
pub enum CropMode {
    /// Creates an asset with the exact specified width and AspectRatio without distorting the asset. This option first
    /// scales as much as needed to fill the specified dimensions. If the requested aspect ratio is
    /// different than the original, cropping will occur on the dimension that exceeds the requested size after
    /// scaling. You can specify which part of the original asset you want to keep if cropping occurs using the
    /// gravity (set to 'center' by default).
    FillByWidth {
        width: u32,
        ar: Option<AspectRatio>,
        gravity: Option<Gravity>,
    },
    /// Same as FillByWidth, but uses the original height instead of the original width to calculate the aspect ratio.
    FillByHeight {
        height: u32,
        ar: Option<AspectRatio>,
        gravity: Option<Gravity>,
    },
    /// Creates an asset with the exact specified width and height without distorting the asset. This option first
    /// scales as much as needed to fill the specified dimensions. If the requested aspect ratio is
    /// different than the original, cropping will occur on the dimension that exceeds the requested size after
    /// scaling. You can specify which part of the original asset you want to keep if cropping occurs using the
    /// gravity (set to 'center' by default).
    Fill {
        width: u32,
        height: u32,
        gravity: Option<Gravity>,
    },
}

impl Display for CropMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CropMode::FillByWidth { width, ar, gravity } => write!(
                f,
                "{}c_fill{},w_{}",
                ar.as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                gravity
                    .as_ref()
                    .map(|g| format!(",{}", g))
                    .unwrap_or("".into()),
                width,
            ),
            CropMode::FillByHeight {
                height,
                ar,
                gravity,
            } => write!(
                f,
                "{}c_fill{},h_{}",
                ar.as_ref()
                    .map(|ar| format!("{},", ar))
                    .unwrap_or("".into()),
                gravity
                    .as_ref()
                    .map(|g| format!(",{}", g))
                    .unwrap_or("".into()),
                height,
            ),
            CropMode::Fill {
                width,
                height,
                gravity,
            } => write!(
                f,
                "c_fill{},w_{},h_{}",
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fill_by_width() {
        assert_eq!(
            CropMode::FillByWidth {
                width: 100,
                ar: None,
                gravity: None,
            }
            .to_string(),
            "c_fill,w_100"
        );
        assert_eq!(
            CropMode::FillByWidth {
                width: 100,
                ar: Some(AspectRatio::Ignore),
                gravity: None,
            }
            .to_string(),
            "fl_ignore_aspect_ratio,c_fill,w_100"
        );

        assert_eq!(
            CropMode::FillByWidth {
                width: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                gravity: Some(Gravity::North),
            }
            .to_string(),
            "ar_16:9,c_fill,g_north,w_100"
        );
    }

    #[test]
    fn test_fill_by_height() {
        assert_eq!(
            CropMode::FillByHeight {
                height: 100,
                ar: None,
                gravity: None,
            }
            .to_string(),
            "c_fill,h_100"
        );
        assert_eq!(
            CropMode::FillByHeight {
                height: 100,
                ar: Some(AspectRatio::Ignore),
                gravity: None,
            }
            .to_string(),
            "fl_ignore_aspect_ratio,c_fill,h_100"
        );

        assert_eq!(
            CropMode::FillByHeight {
                height: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                gravity: Some(Gravity::North),
            }
            .to_string(),
            "ar_16:9,c_fill,g_north,h_100"
        );
    }

    #[test]
    fn test_fill() {
        assert_eq!(
            CropMode::Fill {
                width: 100,
                height: 100,
                gravity: None,
            }
            .to_string(),
            "c_fill,w_100,h_100"
        );
        assert_eq!(
            CropMode::Fill {
                width: 100,
                height: 100,
                gravity: Some(Gravity::AutoClassic),
            }
            .to_string(),
            "c_fill,g_auto:classic,w_100,h_100"
        );
    }
}
