use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawConvert {
    Aspose,
    GoogleSpeech,
    ExtractText,
}

impl fmt::Display for RawConvert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawConvert::Aspose => write!(f, "aspose"),
            RawConvert::GoogleSpeech => write!(f, "google_speech"),
            RawConvert::ExtractText => write!(f, "extract_text"),
        }
    }
}
