use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryType {
    Upload,
    Private,
    Authenticated,
}

impl fmt::Display for DeliveryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeliveryType::Upload => write!(f, "upload"),
            DeliveryType::Private => write!(f, "private"),
            DeliveryType::Authenticated => write!(f, "authenticated"),
        }
    }
}
