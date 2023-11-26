use std::fmt::{Display, Formatter};

use super::named_color::NamedColor;

#[derive(Debug, Clone)]
pub enum AutoModes {
    /// Selects the predominant color, taking only the image border pixels into account.
    Border,
    /// Selects the predominant color while taking all pixels in the image into account.
    Predominant,
    /// Selects the strongest contrasting color to the predominant color, taking only the image border pixels into account.
    BorderContrast,
    /// Selects the strongest contrasting color to the predominant color while taking all pixels in the image into account.
    PredominantContrast,

    /// To automatically apply a gradient fade to the background with multiple colors:

    /// Bases the gradient fade effect on the predominant colors in the image.
    PredominantGradient,
    /// Bases the gradient fade effect on the colors that contrast the predominant colors in the image.
    PredominantGradientContrast,
    /// Bases the gradient fade effect on the predominant colors in the border pixels of the image.
    BorderGradient,
    /// Bases the gradient fade effect on the colors that contrast the predominant colors in the border pixels of the image.
    BorderGradientContrast,
}

impl Display for AutoModes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoModes::Border => write!(f, "border"),
            AutoModes::Predominant => write!(f, "predominant"),
            AutoModes::BorderContrast => write!(f, "border_contrast"),
            AutoModes::PredominantContrast => write!(f, "predominant_contrast"),
            AutoModes::PredominantGradient => write!(f, "predominant_gradient"),
            AutoModes::PredominantGradientContrast => write!(f, "predominant_gradient_contrast"),
            AutoModes::BorderGradient => write!(f, "border_gradient"),
            AutoModes::BorderGradientContrast => write!(f, "border_gradient_contrast"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Two,
    Four,
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Two => write!(f, "2"),
            Number::Four => write!(f, "4"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
    DiagonalDesc,
    DiagonalAsc,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Horizontal => write!(f, "horizontal"),
            Direction::Vertical => write!(f, "vertical"),
            Direction::DiagonalDesc => write!(f, "diagonal_desc"),
            Direction::DiagonalAsc => write!(f, "diagonal_asc"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Named(NamedColor),
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Named(color) => write!(f, "{}", color),
            Color::RGB(r, g, b) => write!(f, "rgb:{:x?}{:x?}{:x?}", r, g, b),
            Color::RGBA(r, g, b, a) => write!(f, "rgb:{:x?}{:x?}{:x?}{:x?}", r, g, b, a),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Auto {
    /// The method to use for determining the solid or gradient color(s) to apply.
    /// Default: `AutoModes::Border`
    pub mode: Option<AutoModes>,
    /// Relevant only when setting mode to one of the 'gradient' options. The number of predominant colors to select.
    /// Default: `Number::Two`
    pub number: Option<Number>,
    /// Relevant only when setting mode to one of the 'gradient' options and when 2 colors are selected for the number
    /// option. Specifies the direction to blend the 2 colors together. (If 4 colors are selected, each gets
    /// interpolated between the four corners and this parameter is ignored.)
    /// Default: `Direction::Horizontal`
    pub direction: Option<Direction>,
    /// The palette of colors to use in the border.
    pub palette: Option<Vec<Color>>,
}

impl Display for Auto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![];
        if let Some(mode) = &self.mode {
            params.push(mode.to_string());
        }
        if let Some(number) = &self.number {
            params.push(number.to_string());
        }
        if let Some(direction) = &self.direction {
            params.push(direction.to_string());
        }
        if let Some(palette) = &self.palette {
            params.push(format!(
                "palette_{}",
                palette
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("_")
            ));
        }
        write!(f, "auto:{}", params.join(":"))
    }
}

/// Applies a background to empty or transparent areas.
///
/// Can also be used as a qualifier to override the default background color for padded cropping, text overlays and
/// generated waveform images.
#[derive(Debug, Clone)]
pub enum Background {
    Color(Color),
    Auto(Auto),
}

impl Display for Background {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Background::Color(color) => write!(f, "b_{}", color),
            Background::Auto(auto) => write!(f, "b_{}", auto),
        }
    }
}
