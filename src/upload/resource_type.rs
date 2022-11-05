use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceTypes {
    Image,
    Raw,
    Video,
    Auto,
}

impl fmt::Display for ResourceTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceTypes::Image => write!(f, "image"),
            ResourceTypes::Raw => write!(f, "raw"),
            ResourceTypes::Video => write!(f, "video"),
            ResourceTypes::Auto => write!(f, "auto"),
        }
    }
}
