use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedHeaders {
    Link,
    Authorization,
    XRobotsTag,
}
impl fmt::Display for AllowedHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllowedHeaders::Link => write!(f, "Link"),
            AllowedHeaders::Authorization => write!(f, "Authorization"),
            AllowedHeaders::XRobotsTag => write!(f, "X-Robots-Tag"),
        }
    }
}
