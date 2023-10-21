use std::fmt::Display;

/// A qualifier that determines which part of an asset to focus on, and thus which part of the asset to keep, when any
/// part of the asset is cropped. For overlays, this setting determines where to place the overlay.
#[derive(Debug, Clone)]
pub enum Gravity {
    /// The compass direction represents a location in the asset, such as the top-right corner.
    NorthEast,
    /// The compass direction represents a location in the asset, such as the top-center.
    North,
    /// The compass direction represents a location in the asset, such as the top-left corner.
    NorthWest,
    /// The compass direction represents a location in the asset, such as the right-center.
    West,
    /// The compass direction represents a location in the asset, such as the bottom-left corner.
    SouthWest,
    /// The compass direction represents a location in the asset, such as the bottom-center.
    South,
    /// The compass direction represents a location in the asset, such as the bottom-right corner.
    SouthEast,
    /// The compass direction represents a location in the asset, such as the left-center.
    East,
    /// The compass direction represents the center of the asset.
    Center,
    /// Automatically detect all eyes in an image with the Advanced Facial Attribute Detection add-on and make them
    /// the focus of the transformation.
    AdvEyes,
    /// Automatically detect the largest face in an image with the Advanced Facial Attribute Detection add-on and make
    /// it the focus of the transformation.
    AdvFace,
    /// Automatically detect all faces in an image with the Advanced Facial Attribute Detection add-on and make them
    /// the focus of the transformation.
    AdvFaces,
    /// Use custom coordinates that were previously specified (e.g., as part of the image upload method) and make them
    /// the focus of the transformation. Defaults to center gravity if no custom coordinates have been specified.
    Custom,
    /// Same as Custom gravity, but defaults to face gravity if no custom coordinates have been specified.
    CustomFace,
    /// Same as Custom gravity, but defaults to AdvFace gravity if no custom coordinates have been specified.
    CustomAdvFace,
    /// Same as Custom gravity, but defaults to AdvFaces gravity if no custom coordinates have been specified.
    CustomAdvFaces,
    /// Same as Custom gravity, but defaults to Faces gravity if no custom coordinates have been specified.
    CustomFaces,
    /// Automatically detect the largest face in an image and make it the focus of the transformation. Any previously
    /// specified face coordinates (during upload, with the Admin API, or via the Cloudinary Console) override the
    /// automatically detected faces and are used instead. Defaults to North gravity if no face is detected or
    /// previously specified. You can also use AutoFace or FaceCenter so that the gravity will default to auto or
    /// center if no face is detected or specified.
    Face,
    ///  Same as Face gravity, but defaults to Center gravity if no face is detected.
    FaceCenter,
    /// Same as Face gravity, but defaults to Auto gravity if no face is detected.
    FaceAuto,
    /// Same as Face gravity, but detects all the faces in an image and uses the rectangle containing all face
    /// coordinates as the basis of the transformation. Any previously specified face coordinates (during upload, with
    /// the Admin API, or via the Cloudinary Console) override the automatically detected faces and are used instead.
    /// Defaults to North gravity if no faces are detected or previously specified. You can also use AutoFaces or
    /// FacesCenter so that the gravity will default to auto or center if no faces are detected or specified.
    Faces,
    /// Same as Faces gravity, but defaults to Center gravity if no face is detected.
    FacesCenter,
    /// Same as Faces gravity, but defaults to Auto gravity if no face is detected.
    FacesAuto,
    /// Detect all text elements in an image using the OCR Text Detection and Extraction add-on and use the detected
    /// bounding box coordinates as the basis of the transformation.
    OcrText,
    /// Applies deep-learning algorithms to identify the subjects of an image that are most likely to attract a
    /// person's gaze.
    AutoSubject,
    /// Uses a combination of saliency heuristics to automatically detect significant regions in the image.
    AutoClassic,
}

impl Display for Gravity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gravity::NorthEast => write!(f, "g_north_east"),
            Gravity::North => write!(f, "g_north"),
            Gravity::NorthWest => write!(f, "g_north_west"),
            Gravity::West => write!(f, "g_west"),
            Gravity::SouthWest => write!(f, "g_south_west"),
            Gravity::South => write!(f, "g_south"),
            Gravity::SouthEast => write!(f, "g_south_east"),
            Gravity::East => write!(f, "g_east"),
            Gravity::Center => write!(f, "g_center"),
            Gravity::AdvEyes => write!(f, "g_adv_eyes"),
            Gravity::AdvFace => write!(f, "g_adv_face"),
            Gravity::AdvFaces => write!(f, "g_adv_faces"),
            Gravity::Custom => write!(f, "g_custom"),
            Gravity::CustomFace => write!(f, "g_custom:face"),
            Gravity::CustomAdvFace => write!(f, "g_custom:adv_face"),
            Gravity::CustomAdvFaces => write!(f, "g_custom:adv_faces"),
            Gravity::CustomFaces => write!(f, "g_custom:faces"),
            Gravity::Face => write!(f, "g_face"),
            Gravity::FaceCenter => write!(f, "g_face:center"),
            Gravity::FaceAuto => write!(f, "g_face:auto"),
            Gravity::Faces => write!(f, "g_faces"),
            Gravity::FacesCenter => write!(f, "g_faces:center"),
            Gravity::FacesAuto => write!(f, "g_faces:auto"),
            Gravity::OcrText => write!(f, "g_ocr_text"),
            Gravity::AutoSubject => write!(f, "g_auto:subject"),
            Gravity::AutoClassic => write!(f, "g_auto:classic"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transformation::gravity::Gravity;

    #[test]
    fn test_gravity_to_string() {
        assert_eq!(Gravity::NorthEast.to_string(), "g_north_east");
        assert_eq!(Gravity::North.to_string(), "g_north");
        assert_eq!(Gravity::NorthWest.to_string(), "g_north_west");
        assert_eq!(Gravity::West.to_string(), "g_west");
        assert_eq!(Gravity::SouthWest.to_string(), "g_south_west");
        assert_eq!(Gravity::South.to_string(), "g_south");
        assert_eq!(Gravity::SouthEast.to_string(), "g_south_east");
        assert_eq!(Gravity::East.to_string(), "g_east");
        assert_eq!(Gravity::Center.to_string(), "g_center");
        assert_eq!(Gravity::AdvEyes.to_string(), "g_adv_eyes");
        assert_eq!(Gravity::AdvFace.to_string(), "g_adv_face");
        assert_eq!(Gravity::AdvFaces.to_string(), "g_adv_faces");
        assert_eq!(Gravity::Custom.to_string(), "g_custom");
        assert_eq!(Gravity::CustomFace.to_string(), "g_custom:face");
        assert_eq!(Gravity::CustomAdvFace.to_string(), "g_custom:adv_face");
        assert_eq!(Gravity::CustomAdvFaces.to_string(), "g_custom:adv_faces");
        assert_eq!(Gravity::CustomFaces.to_string(), "g_custom:faces");
        assert_eq!(Gravity::Face.to_string(), "g_face");
        assert_eq!(Gravity::FaceCenter.to_string(), "g_face:center");
        assert_eq!(Gravity::FaceAuto.to_string(), "g_face:auto");
        assert_eq!(Gravity::Faces.to_string(), "g_faces");
        assert_eq!(Gravity::FacesCenter.to_string(), "g_faces:center");
        assert_eq!(Gravity::FacesAuto.to_string(), "g_faces:auto");
        assert_eq!(Gravity::OcrText.to_string(), "g_ocr_text");
        assert_eq!(Gravity::AutoSubject.to_string(), "g_auto:subject");
        assert_eq!(Gravity::AutoClassic.to_string(), "g_auto:classic");
    }
}
