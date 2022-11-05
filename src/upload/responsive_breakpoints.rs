use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ResponsiveBreakpoints {
    create_derived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_step: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_images: Option<u32>,
}

impl ResponsiveBreakpoints {
    pub fn new(create_derived: bool) -> Self {
        Self {
            create_derived,
            format: None,
            transformation: None,
            max_width: None,
            min_width: None,
            bytes_step: None,
            max_images: None,
        }
    }
}
