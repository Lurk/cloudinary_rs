use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ResponsiveBreakpoints {
    /// If true, create and keep the derived images of the selected breakpoints during the API call. If false, images
    /// generated during the analysis process are thrown away.
    pub create_derived: bool,
    /// Sets the file extension of the derived assets to the format indicated (as opposed to changing the format as
    /// part of a transformation - which would be included as part of the transformation component (e.g., f_jpg)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The base transformation to first apply to the image before finding the best breakpoints. The API accepts a
    /// string representation of a chained transformation (same as the regular transformation parameter of the upload
    /// API).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation: Option<String>,
    /// The maximum width needed for this image. If specifying a width bigger than the original image, the width of the
    /// original image is used instead.
    ///
    /// Default: 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<u32>,
    /// The minimum width needed for this image.
    ///
    /// Default: 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<u32>,
    /// The minimum number of bytes between two consecutive breakpoints (images).
    ///
    /// Default: 20000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_step: Option<u32>,
    /// The maximum number of breakpoints to find, between 3 and 200. This means that there might be size differences
    /// bigger than the given bytes_step value between consecutive images.
    ///
    /// Default: 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<u32>,
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
