use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryType {
    Upload,
    Private,
    Authenticated,
    List,
    Fetch,
    Facebook,
    Twitter,
    TwitterName,
    Gravatar,
    Youtube,
    Hulu,
    Vimeo,
    Animoto,
    Worldstarhiphop,
    Dailymotion,
    Multi,
    Text,
    Asset,
}

impl fmt::Display for DeliveryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeliveryType::Upload => write!(f, "upload"),
            DeliveryType::Private => write!(f, "private"),
            DeliveryType::Authenticated => write!(f, "authenticated"),
            DeliveryType::List => write!(f, "list"),
            DeliveryType::Fetch => write!(f, "fetch"),
            DeliveryType::Facebook => write!(f, "facebook"),
            DeliveryType::Twitter => write!(f, "twitter"),
            DeliveryType::TwitterName => write!(f, "twitter_name"),
            DeliveryType::Gravatar => write!(f, "gravatar"),
            DeliveryType::Youtube => write!(f, "youtube"),
            DeliveryType::Hulu => write!(f, "hulu"),
            DeliveryType::Vimeo => write!(f, "vimeo"),
            DeliveryType::Animoto => write!(f, "animoto"),
            DeliveryType::Worldstarhiphop => write!(f, "worldstarhiphop"),
            DeliveryType::Dailymotion => write!(f, "dailymotion"),
            DeliveryType::Multi => write!(f, "multi"),
            DeliveryType::Text => write!(f, "text"),
            DeliveryType::Asset => write!(f, "asset"),
        }
    }
}
