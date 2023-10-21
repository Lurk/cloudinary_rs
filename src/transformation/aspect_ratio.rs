use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
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
