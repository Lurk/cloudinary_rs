use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedHeaders {
    /// [Link HTTP header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Link)
    ///
    /// TODO: better type
    Link(String),
    /// [Authorization HTTP header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization)
    ///
    /// TODO: better type
    Authorization(String),
    /// [X-Robots-Tag HTTP header](https://developers.google.com/search/docs/crawling-indexing/robots-meta-tag#xrobotstag)
    ///
    /// TODO: better type
    XRobotsTag(String),
}
impl fmt::Display for AllowedHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllowedHeaders::Link(s) => write!(f, "Link: {}", s),
            AllowedHeaders::Authorization(s) => write!(f, "Authorization: {}", s),
            AllowedHeaders::XRobotsTag(s) => write!(f, "X-Robots-Tag: {}", s),
        }
    }
}
