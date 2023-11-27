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

#[cfg(test)]
mod test {
    use crate::transformation::{
        aspect_ratio::AspectRatio,
        background::{Auto, AutoModes, Color, Direction, Number},
        gravity::Gravity,
        named_color::NamedColor,
        pad_mode::PadMode,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn pad_by_width() {
        assert_eq!(
            PadMode::PadByWidth {
                width: 100,
                ar: None,
                background: None,
                gravity: None,
            }
            .to_string(),
            "c_pad,w_100"
        );
        assert_eq!(
            PadMode::PadByWidth {
                width: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                background: None,
                gravity: None,
            }
            .to_string(),
            "ar_16:9,c_pad,w_100"
        );
        assert_eq!(
            PadMode::PadByWidth {
                width: 100,
                ar: None,
                background: Some(NamedColor::Black.into()),
                gravity: None,
            }
            .to_string(),
            "b_black,c_pad,w_100"
        );
        assert_eq!(
            PadMode::PadByWidth {
                width: 100,
                ar: None,
                background: None,
                gravity: Some(Gravity::North),
            }
            .to_string(),
            "c_pad,g_north,w_100"
        );
        assert_eq!(
            PadMode::PadByWidth {
                width: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                background: Some(Color::RGB(0, 0, 0).into()),
                gravity: Some(Gravity::North),
            }
            .to_string(),
            "b_rgb:000000,ar_16:9,c_pad,g_north,w_100"
        );
        assert_eq!(
            PadMode::PadByWidth {
                width: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                background: Some(
                    Auto {
                        mode: Some(AutoModes::BorderGradient),
                        number: Some(Number::Four),
                        direction: Some(Direction::Vertical),
                        palette: Some(vec![NamedColor::Black.into(), NamedColor::White.into()])
                    }
                    .into()
                ),
                gravity: Some(Gravity::North),
            }
            .to_string(),
            "b_auto:border_gradient:4:vertical:palette_black_white,ar_16:9,c_pad,g_north,w_100"
        );
    }

    #[test]
    fn pad_by_height() {
        assert_eq!(
            PadMode::PadByHeight {
                height: 100,
                ar: None,
                background: None,
                gravity: None,
            }
            .to_string(),
            "c_pad,h_100"
        );
        assert_eq!(
            PadMode::PadByHeight {
                height: 100,
                ar: Some(AspectRatio::Result(0.5)),
                background: None,
                gravity: None,
            }
            .to_string(),
            "ar_0.5,c_pad,h_100"
        );
        assert_eq!(
            PadMode::PadByHeight {
                height: 100,
                ar: None,
                background: Some(NamedColor::MediumTurquoise.into()),
                gravity: None,
            }
            .to_string(),
            "b_mediumturquoise,c_pad,h_100"
        );
        assert_eq!(
            PadMode::PadByHeight {
                height: 100,
                ar: None,
                background: None,
                gravity: Some(Gravity::FaceCenter),
            }
            .to_string(),
            "c_pad,g_face:center,h_100"
        );
        assert_eq!(
            PadMode::PadByHeight {
                height: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                background: Some(Color::RGBA(0, 0, 0, 10).into()),
                gravity: Some(Gravity::SouthEast),
            }
            .to_string(),
            "b_rgb:0000000a,ar_16:9,c_pad,g_south_east,h_100"
        );
        assert_eq!(
            PadMode::PadByHeight {
                height: 100,
                ar: Some(AspectRatio::Sides(16, 9)),
                background: Some(
                    Auto {
                        mode: Some(AutoModes::Border),
                        number: Some(Number::Two),
                        direction: Some(Direction::Horizontal),
                        palette: Some(vec![NamedColor::Brown.into(), NamedColor::BurlyWood.into()])
                    }
                    .into()
                ),
                gravity: Some(Gravity::NorthEast),
            }
            .to_string(),
            "b_auto:border:2:horizontal:palette_brown_burlywood,ar_16:9,c_pad,g_north_east,h_100"
        );
    }

    #[test]
    fn pad() {
        assert_eq!(
            PadMode::Pad {
                width: 100,
                height: 100,
                background: None,
                gravity: None,
            }
            .to_string(),
            "c_pad,w_100,h_100"
        );
        assert_eq!(
            PadMode::Pad {
                width: 100,
                height: 100,
                background: Some(NamedColor::MediumPurple.into()),
                gravity: None,
            }
            .to_string(),
            "b_mediumpurple,c_pad,w_100,h_100"
        );
        assert_eq!(
            PadMode::Pad {
                width: 100,
                height: 100,
                background: None,
                gravity: Some(Gravity::FaceAuto),
            }
            .to_string(),
            "c_pad,g_face:auto,w_100,h_100"
        );
        assert_eq!(
            PadMode::Pad {
                width: 100,
                height: 100,
                background: Some(Color::RGBA(0, 1, 0, 10).into()),
                gravity: Some(Gravity::AutoClassic),
            }
            .to_string(),
            "b_rgb:0001000a,c_pad,g_auto:classic,w_100,h_100"
        );
        assert_eq!(
            PadMode::Pad {
                width: 100,
                height: 100,
                background: Some(
                    Auto {
                        mode: Some(AutoModes::PredominantGradientContrast),
                        number: None,
                        direction: None,
                        palette: Some(vec![NamedColor::Azure.into(), NamedColor::SkyBlue.into()])
                    }
                    .into()
                ),
                gravity: Some(Gravity::CustomFace),
            }
            .to_string(),
            "b_auto:predominant_gradient_contrast:palette_azure_skyblue,c_pad,g_custom:face,w_100,h_100"
        );
    }
}
