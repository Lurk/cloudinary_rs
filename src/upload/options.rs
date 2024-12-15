use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

use itertools::Itertools;
use url::Url;

use crate::transformation::Transformations;

use super::raw_convert::RawConvert;
use super::{
    access_mode::AccessModes, allowed_headers::AllowedHeaders,
    background_removal::BackgroundRemoval, categorizations::Categorizations,
    delivery_type::DeliveryType, moderation::Moderation, resource_type::ResourceTypes,
    responsive_breakpoints::ResponsiveBreakpoints,
};

/// Image upload optional parameters from
/// [cloudinary docs](https://cloudinary.com/documentation/image_upload_api_reference#upload_optional_parameters)
pub enum OptionalParameters {
    /// The identifier that's used for accessing and delivering the uploaded asset.
    ///
    /// If not specified, then the public ID of the asset will either be comprised of random
    /// characters or will use the original file's filename, depending whether use_filename was set to true.
    ///
    /// **NOTE:**
    ///
    /// - The public ID value for images and videos should not include a file extension. Include
    ///     the file extension for raw files only.
    /// - Can be up to 255 characters, including:
    ///     - non-English characters
    ///     - periods `.`
    ///     - forward slashes `/`
    ///     - underscores `_`
    ///     - hyphens `-`
    /// - [Public ID](Options::PublicId) values cannot begin or end with a space or forward slash `/`. Additionally,
    ///     they cannot include the following characters: `? & # \ % < > +`
    PublicId(String),
    /// A string or path that's automatically prepended to the public_id with a forward slash. The
    /// value can contain the same characters as the public_id including additional forward
    /// slashes. This prefix can be useful to provide context and improve the SEO of an asset's filename in the
    /// delivery URL, but the value does not impact the location where the asset is stored.
    ///
    /// Not relevant for product environments using the legacy
    /// [fixed folder mode](https://cloudinary.com/documentation/folder_modes).
    PublicIdPrefix(String),
    /// A user-friendly name for the asset.
    ///
    /// Default: Same value as the public ID (or the last segment of the
    /// public ID if the public ID includes slashes).
    ///
    /// Display names can have spaces and special characters, but can't include forward slashes `/`. This name can be
    /// completely different than the asset's public ID and its value doesn't impact the delivery
    /// URL in any way.
    ///
    /// The display name is shown in user interface pages such as the Console Media Explorer, Cloudinary collections,
    /// and Cloudinary media portals.
    ///
    /// Though not a best practice, it's possible for the same display name to be used for different assets, even in
    /// the same asset folder.
    ///
    /// Not relevant for product environments using the legacy
    /// [fixed folder mode](https://cloudinary.com/documentation/folder_modes).
    DisplayName(String),
    /// The folder where the asset is placed within the Cloudinary repository. This value does not impact the asset’s
    /// public ID path (unless the
    /// [use_asset_folder_as_public_id_prefix](Options::UseAssetFolderAsPublicIdPrefix) option is applied).
    ///
    /// Default: If not specified, the uploaded asset will be located in the root of your product environment asset
    /// repository, even if the public ID value includes slashes.
    ///
    /// Not relevant for product environments using the legacy
    /// [fixed folder mode](https://cloudinary.com/documentation/folder_modes).
    AssetFolder(String),
    /// Whether to add the asset_folder value as a prefix to the public_id value (prepended with a
    /// forward slash). This ensures that the public ID path will always match the initial asset
    /// folder, and can help to retain the behavior that previously existed in fixed folder mode. However, keep in mind
    /// that even when this option is used during upload, an asset with a certain public ID path
    /// can later be moved to a completely different asset folder hierarchy without impacting the
    /// public ID. This option only ensures path matching for the initial upload.
    ///
    /// Relevant only when [public_id_prefix](Options::PublicIdPrefix) (or [folder](Options::AssetFolder)) has not been
    /// separately specified.
    ///
    /// Default: false
    ///
    /// Not relevant for product environments using the legacy
    /// [fixed folder mode](https://cloudinary.com/documentation/folder_modes).
    UseAssetFolderAsPublicIdPrefix(bool),
    /// Only relevant for product environments using the legacy
    /// [fixed folder mode](https://cloudinary.com/documentation/folder_modes).
    ///
    /// Defines both the full path of the folder where the uploaded asset will be placed and also a path value that's
    /// prepended to public_id value with a forward slash.
    ///
    /// Default: root folder.
    ///
    /// NOTE: If Dynamic folders mode is enabled on your product environment, this parameter is deprecated, and it's
    /// recommended to use the asset_folder parameter to control where the asset will be placed. If you also want your
    /// public_id to match the initial asset folder path, include the use_asset_folder_as_public_id_prefixparameter.
    Folder(String),
    /// Whether to use the original file name of the uploaded file as the public_id. Relevant only if the public_id
    /// parameter isn't set.
    ///
    /// When false and the public_id parameter is also not defined, the public ID will be comprised of random
    /// characters.
    ///
    /// When true and the public_id parameter is not defined, the uploaded file's original filename becomes the
    /// public ID. Random characters are appended to the filename value to ensure public ID uniqueness if
    /// unique_filename is true.
    ///
    /// If the filename of the asset you upload contains a character that's not supported for public IDs,
    /// preceding/trailing occurrences are trimmed off, while illegal characters anywhere else in the filename are
    /// replaced with underscores.
    ///
    /// Default: false.
    UseFileName(bool),
    /// Whether to automatically assign the original filename of the uploaded asset as the asset's display name.
    /// Relevant only if the display_name parameter isn't set.
    ///
    /// Note: If you set use_filename_as_display_name to true (in the upload call or upload preset) and the original
    /// filename of the asset includes forward slashes, the upload will fail with an error that the display name can't
    /// include slashes.
    ///
    /// Default: false.
    ///
    /// Not relevant for product environments using the legacy
    /// [fixed folder mode](https://cloudinary.com/documentation/folder_modes).
    UseFilenameAsDisplayName(bool),
    /// When set to false, does not add random characters at the end of the filename that guarantee its uniqueness.
    /// Note that if the overwrite parameter is also false, the public ID will be comprised of random characters. This
    /// parameter is relevant only if use_filename is also set to `true`.
    ///
    /// Default: true.
    UniqueFilename(bool),
    /// Sets the 'original-filename' metadata header stored on the asset (instead of using the actual filename of the
    /// uploaded file). Useful together with the use_filename parameter and for advanced search by filename, and
    /// relevant when delivering assets as attachments (setting the flag transformation parameter to attachment).
    FilenameOverride(String),
    /// Set the type of file you are uploading or use auto to automatically detect the file type. Only relevant as a
    /// parameter when using the SDKs (the resource_type is included in the endpoint URL when using the REST API).
    /// Valid values: image, raw, video and auto. Defaults: image for server-side uploading (with the exception of
    /// the Go SDK which defaults to auto) and auto for client-side uploading.
    ///
    /// NOTE: Use the video resource type for all video assets as well as for audio files, such as .mp3.
    ResourceType(ResourceTypes),
    /// The delivery type. Allows uploading assets as private or authenticated instead of the default upload mode. Only
    /// relevant as a parameter when using the SDKs (the delivery type value is part of the endpoint URL when using the
    /// REST API). Valid values: upload, private and authenticated.
    ///
    /// Default: upload.
    Type(DeliveryType),
    /// TODO: better type.
    ///
    /// Restrict access to the asset by passing an array of access types for the asset. The asset is restricted unless
    /// one of the access types is valid.
    ///
    /// Possible values for each access type:
    ///
    /// - token requires either Token-based access or Cookie-based access for accessing the asset.
    ///     For example: access_type: "token"
    /// - anonymous allows public access to the asset. The anonymous access type can optionally include start and/or
    ///     end dates (in ISO 8601 format) that define when the asset is publicly available. Note that you can only
    ///     include a single 'anonymous' access type.
    ///     For example: `access_type: "anonymous", start: "2017-12-15T12:00Z", end: "2018-01-20T12:00Z"`
    AccessControl(String),
    /// Allows the asset to behave as if it's of the authenticated 'type' (see above) while still using the default
    /// 'upload' type in delivery URLs. The asset can later be made public by changing its access_mode via the
    /// [Admin API](https://cloudinary.com/documentation/admin_api#update_access_mode), without having to update any
    /// delivery URLs. Valid values: public, and authenticated.
    ///
    /// Default: public.
    AccessMode(AccessModes),
    /// Whether to discard the name of the original uploaded file. Relevant when delivering assets as attachments
    /// (setting the flag transformation parameter to attachment).
    ///
    /// Default: false.
    DiscardOriginalFilename(bool),
    /// Whether to overwrite existing assets with the same public ID. When set to false, a response is returned
    /// immediately if an asset with the same public ID was found.
    ///
    /// When overwriting assets, if you include versions in your delivery URLs, you will need to update the URLs with
    /// the new version number to deliver the new asset. If you don't include versions, you will need to invalidate the
    /// old assets on the CDN server cache.
    ///
    /// Default: true (when using unsigned upload, the default is false and cannot be changed to true).
    ///
    /// Important: Depending on your product environment setup, overwriting an asset may clear the tags, contextual,
    /// and structured metadata values for that asset. If you have a Master admin role, you can change this behavior
    /// for your product environment in the Media Library Preferences pane, so that these field values are retained
    /// when new version assets overwrite older ones (unless you specify different values for the tags, context, or
    /// metadata parameters as part of your upload).
    Overwrite(bool),
    /// A comma-separated list of tag names to assign to the uploaded asset for later group reference.
    /// For example: animal,dog
    ///
    /// SDKs: Supports arrays. For example: ['animal', 'dog']
    Tags(HashSet<String>),
    /// A pipe-separated list of the key-value pairs of contextual metadata to attach to an uploaded asset. The context
    /// values of uploaded files can be retrieved using the Admin API. For example: alt=My image|caption=Profile image
    ///
    /// NOTE:
    ///
    /// - The = and | characters can be supported as values when escaped with a prepended backslash (\).
    /// - Key values are limited to 1024 characters and an asset can have a maximum of 1000 context key-value pairs.
    ///
    /// SDKs: Supports maps. For example: ['alt': 'My image', 'caption': 'Profile image']
    Context(HashMap<String, String>),
    /// A pipe-separated list of custom metadata fields (by external_id) and the values to assign to each of them. For
    /// example: in_stock_id=50|color_id=[\"green\",\"red\"].
    ///
    /// SDKs: Supports maps.
    ///
    /// NOTE:
    ///
    /// - The =, " and | characters can be supported as values when escaped with a prepended backslash (\).
    /// - For a multi-select field, you can set a maximum of 3000 different metadata values on an asset.
    Metadata(HashMap<String, String>),
    /// (Relevant for cascading metadata, and assets with multiple metadata fields) When updating the value of a
    /// metadata field results in another metadata field’s value becoming invalid, that invalid value is cleared
    /// instead of resulting in an error.
    ///
    /// Default: false.
    ClearInvalid(bool),
    /// Whether to retrieve predominant colors & color histogram of the uploaded image.
    ///
    /// NOTE: If all returned colors are opaque, then 6-digit RGB hex values are returned. If one or more colors
    /// contain an alpha channel, then 8-digit RGBA hex quadruplet values are returned.
    ///
    /// Default: false.
    ///
    /// Relevant for images only.
    Colors(bool),
    /// Whether to return the coordinates of faces contained in an uploaded image (automatically detected or manually
    /// defined). Each face is specified by the X & Y coordinates of the top left corner and the width & height of the
    /// face. The coordinates for each face are returned as an array (using the SDKs) or a comma-separated list (for
    /// REST API calls), and individual faces are separated with a pipe (|).
    ///
    /// For example: 10,20,150,130|213,345,82,61.
    ///
    /// Default: false.
    ///
    /// Relevant for images only.
    Faces(bool),
    /// Whether to return a quality analysis value for the image between 0 and 1, where 0 means the image is blurry and
    /// out of focus and 1 means the image is sharp and in focus.
    ///
    /// Default: false.
    ///
    /// Relevant for images only.
    ///
    /// Paid customers can request to take part in the extended quality analysis Beta trial. When activated, this
    /// parameter returns quality scores for various other factors in addition to focus, such as jpeg_quality, noise,
    /// exposure, lighting and resolution, together with an overall weighted quality_score. The quality_score,
    /// quality_analysis.color_score and quality_analysis.pixel_score fields can be used in the Search API.
    QualityAnalysis(bool),
    /// Currently available only to paid customers requesting to take part in the accessibility analysis Beta trial.
    /// Set to true to return accessibility analysis values for the image and to enable the
    /// accessibility_analysis.colorblind_accessibility_score field to be used in the Search API.
    ///
    /// Default: false.
    ///
    /// Relevant for images only.
    AccessibilityAnalysis(bool),
    /// Whether to return a cinemagraph analysis value for the media asset between 0 and 1, where 0 means the asset is
    /// not a cinemagraph and 1 means the asset is a cinemagraph. Default: false. Relevant for animated images and
    /// video only. A static image will return 0.
    CinemagraphAnalysis(bool),
    /// Whether to return IPTC, XMP, and detailed Exif metadata of the uploaded asset in the response.
    ///
    /// Default: false.
    ///
    /// Supported for images, video, and audio.
    ///
    /// - Returned metadata for images includes: PixelsPerUnitX, PixelsPerUnitY, PixelUnits, Colorspace, and DPI.
    /// - Returned metadata for audio and video includes: audio_codec, audio_bit_rate, audio_frequency, channels,
    ///     channel_layout.
    /// - Additional metadata for video includes: pix_format, codec, level, profile, video_bit_rate, dar.
    ///
    /// (In .NET SDK, parameter name is Metadata.)
    MediaMetadata(bool),
    /// Whether to return the perceptual hash (pHash) on the uploaded image. The pHash acts as a fingerprint that
    /// allows checking image similarity.
    ///
    /// Default: `false`.
    ///
    /// Relevant for images only.
    Phash(bool),
    /// Requests that Cloudinary automatically find the best breakpoints.
    ///
    /// The return response will include an array of the selected breakpoints for each breakpoint request, where the
    /// following information is given for each breakpoint: `transformation`, `width`, `height`, `bytes`, `url` and
    /// `secure_url`.
    ///
    /// Relevant for images only.
    ResponsiveBreakpoints(Vec<ResponsiveBreakpoints>),
    /// Automatically assigns tags to an asset according to detected objects or categories with a confidence score
    /// higher than the specified value.
    ///
    /// Use together with the detection parameter for:
    /// - [Cloudinary AI Content Analysis](https://cloudinary.com/documentation/cloudinary_ai_content_analysis_addon#automatic_image_tagging)
    /// - []Amazon Rekognition Celebrity Detection](https://cloudinary.com/documentation/aws_rekognition_celebrity_and_face_detection_addon#automatically_adding_tags_to_images)
    ///
    /// Use together with the categorization parameter for:
    ///
    /// - [Google Automatic Video Tagging](https://cloudinary.com/documentation/google_automatic_video_tagging_addon#adding_resource_tags_to_videos)
    /// - [Google Auto Tagging](https://cloudinary.com/documentation/google_auto_tagging_addon#adding_resource_tags_to_images)
    /// - [Imagga Auto Tagging](https://cloudinary.com/documentation/imagga_auto_tagging_addon#adding_resource_tags_to_images)
    /// - [Amazon Rekognition Auto Tagging](https://cloudinary.com/documentation/aws_rekognition_auto_tagging_addon#automatically_adding_tags_to_images)
    ///
    /// Range: `0.0` to `1.0`
    AutoTagging(f64),
    /// A comma-separated list of the categorization add-ons to run on the asset. Set to `google_tagging`,
    /// `google_video_tagging`, `imagga_tagging` and/or `aws_rek_tagging` to automatically classify the scenes of the
    /// uploaded asset. Can be used together with the auto_tagging parameter to apply tags automatically. See the
    /// [Google Automatic Video Tagging](https://cloudinary.com/documentation/google_automatic_video_tagging_addon#adding_resource_tags_to_videos),
    /// [Google Auto Tagging](https://cloudinary.com/documentation/google_auto_tagging_addon#adding_resource_tags_to_images),
    /// [Imagga Auto Tagging](https://cloudinary.com/documentation/imagga_auto_tagging_addon#adding_resource_tags_to_images)
    /// and
    /// [Amazon Rekognition Auto Tagging](https://cloudinary.com/documentation/aws_rekognition_auto_tagging_addon#automatically_adding_tags_to_images)
    /// add-ons for more details.
    Categorization(Categorizations),
    /// Invokes the relevant add-on to return a list of detected content.
    ///
    /// Set to:
    ///
    /// - <content-aware model>_[<version>] (e.g. coco_v2) to return a list of detected content using the
    ///     [Cloudinary AI Content Analysis add-on](https://cloudinary.com/documentation/cloudinary_ai_content_analysis_addon#automatic_image_tagging).
    ///     Can be used together with the auto_tagging parameter to apply tags automatically.
    /// - `captioning` to analyze an image and suggest a caption based on the image's contents.
    /// - `iqa` to analyze the quality of an image.
    /// - `adv_face` to return a list of facial attributes using the Advanced Facial Attribute Detection add-on.
    /// - `aws_rek_face` to return a list of detected celebrities and facial attributes using the Amazon Rekognition
    ///     Celebrity Detection add-on. Can be used together with the `auto_tagging` parameter to apply
    ///     tags automatically.
    ///
    /// Relevant for images only.
    Detection(String),
    /// Whether to trigger automatic generation of video chapters. Chapters will be generated and saved as a .vtt file
    /// with -chapters appended to the public ID of the video. You can enable chapters as part of the
    /// [Cloudinary Video Player](https://cloudinary.com/documentation/video_player_customization#video_chapters).
    ///
    /// Default: false.
    ///
    /// Relevant for videos only.
    AutoChaptering(bool),
    /// Whether to trigger automatic video transcription. The transcript will be generated and saved as a .transcript
    /// file with the same public ID as the video. You can use your transcript file to show subtitles or captions using
    /// the [Cloudinary Video Player](https://cloudinary.com/documentation/video_player_customization#video_chapters).
    ///
    /// Default: false.
    ///
    /// Relevant for videos only.
    AutoTranscription(bool),
    /// Set to adv_ocr to extract all text elements in an image as well as the bounding box coordinates of each
    /// detected element using the
    /// [OCR text detection and extraction add-on](https://cloudinary.com/documentation/ocr_text_detection_and_extraction_addon).
    ///
    /// Relevant for images only.
    Ocr,
    /// Whether to index the image for use with visual searches.
    ///
    /// Default: false.
    ///
    /// Relevant for images only.
    VisualSearch(bool),
    /// A list of transformations to create for the uploaded asset, instead of lazily creating them when first accessed
    /// by your site's visitors (see the
    /// [Transformation URL API Reference](https://cloudinary.com/documentation/transformation_reference) for more
    /// details on possible values). This option accepts either a single transformation or a pipe-separated list of
    /// transformations to create for the uploaded asset.
    ///
    /// SDKs: Supports arrays. (In .NET SDK, parameter name is EagerTransforms.)
    Eager(Vec<Transformations>),
    /// Whether to generate the eager transformations asynchronously in the background after the upload request is
    /// completed rather than online as part of the upload call.
    ///
    /// Default: false.
    EagerAsync(bool),
    /// An HTTP or HTTPS URL to send a notification to (a webhook) when the generation of eager transformations is
    /// completed.
    EagerNotificationUrl(Url),
    /// An incoming transformation to run on the uploaded asset before saving it in the cloud. This parameter is given
    /// as a string of comma-separated single characters (separated with a slash for chained transformations).
    ///
    /// SDKs: Supports a hash of transformation parameters (or an array of hashes for chained transformations).
    ///
    /// NOTE: When using the SDK for a dynamically typed language such as Ruby, the transformation parameters can be
    /// specified directly without using this transformation parameter.
    Transformation(Vec<Transformations>),
    /// An optional format to convert the uploaded asset to before saving in the cloud.
    ///
    /// For example: jpg.
    Format(String),
    /// The coordinates of a region contained in the image being uploaded that can be subsequently used for cropping or
    /// adding layers using the custom gravity mode. The region is specified by the X & Y coordinates of the top left
    /// corner and the width & height of the region, as a comma-separated list. For example: 85,120,220,310.
    ///
    ///Relevant for images only.
    ///
    ///SDKs: Supports arrays. For example: [85, 120, 220, 310].
    CustomCoordinates([u16; 4]),
    /// The coordinates of one or more named regions contained in the image being uploaded that can be subsequently
    /// used for cropping using the region gravity mode. Each region is specified by a name (alphanumeric characters
    /// and hyphens permitted) and an array of at least two X,Y coordinate pairs, e.g.,
    /// `{ "name1": [[1, 2], [3, 4]], "name2": [[5,6], [7,8], [9,10]] }`. If two pairs are specified, these refer to
    /// the top left and bottom right coordinates of a rectangle. Otherwise, if more pairs are specified, they refer to
    /// the corners of a custom region.
    ///
    /// Relevant for images only.
    ///
    /// TODO: find a way to check `at least two X,Y coordinate pairs` requirement at compile time
    Regions(HashMap<String, Vec<[u16; 2]>>),
    /// The coordinates of faces contained in an uploaded image to override the automatically detected faces. Each face
    /// is specified by the X & Y coordinates of the top left corner and the width & height of the face. The
    /// coordinates for each face are given as a comma-separated list, with individual faces separated with a
    /// pipe (|)).
    ///
    /// For example: 10,20,150,130|213,345,82,61.
    ///
    /// Relevant for images only.
    ///
    /// SDKs: Supports arrays. For example: [[10, 20, 150, 130],[213, 345, 82, 61]].
    FaceCoordinates(Vec<[u16; 4]>),
    /// Automatically remove the background of an image using an add-on.
    ///
    /// - Set to cloudinary_ai to use the deep-learning based
    ///     [Cloudinary AI Background Removal add-on](https://cloudinary.com/documentation/cloudinary_ai_background_removal_addon#removing_the_background_on_upload_update). NOTE: this feature has been superseded by
    ///     [background removal on the fly](https://cloudinary.com/documentation/cloudinary_ai_background_removal_addon#removing_the_background_on_the_fly).
    /// - Set to pixelz to use the human-powered
    ///     [Pixelz Remove-The-Background Editing add-on service](https://cloudinary.com/documentation/remove_the_background_image_editing_addon).
    ///
    /// Relevant for images only.
    ///
    /// (Asynchronous)
    BackgroundRemoval(BackgroundRemoval),
    /// Generates a related file based on the uploaded file.
    ///
    /// - Set to `aspose` to automatically create a PDF or other image format from a `raw` Office document using the
    ///     [Aspose Document Conversion add-on](https://cloudinary.com/documentation/aspose_document_conversion_addon).
    ///     (Asynchronous)
    /// - Set to `google_speech` to instruct the
    ///     [Google AI Video Transcription](https://cloudinary.com/documentation/google_ai_video_transcription_addon)
    ///     add-on to generate an automatic transcript raw file from an uploaded video. (Asynchronous)
    /// - Set to `extract_text` to extract all the text from a PDF file and store it in a `raw` JSON file with a
    ///     public ID in the format: `[pdf_public_id].extract_text.json`. The full URL of the generated JSON file is
    ///     included in the API response. (Synchronous)
    ///
    /// See also:
    /// [Converting raw files](https://cloudinary.com/documentation/upload_parameters#uploading_non_media_files_as_raw_files).
    RawConvert(RawConvert),
    /// A comma-separated list of file formats that are allowed for uploading. Files of other types will be rejected.
    /// The formats can be any combination of image types, video formats or raw file extensions. For
    /// example: mp4,ogv,jpg,png,pdf.
    ///
    /// Default: any supported format for images and videos, and any kind of raw file (i.e. no restrictions by
    /// default).
    ///
    /// SDKs: Supports arrays. For example: [mp4, ogv, jpg, png, pdf]
    ///
    /// NOTE: You can also add the format parameter to convert other file types instead of rejecting them. In this
    /// case, only files that would normally be rejected are converted, any file format allowed for upload wont be
    /// converted.
    AllowedFormats(Vec<String>),
    /// Tells Cloudinary whether to perform the upload request in the background (asynchronously).
    ///
    /// Default: false.
    ///
    /// NOTE: In the Python SDK, this parameter is passed as a dictionary: **{"async": True}
    Async(bool),
    /// Tell Cloudinary whether to back up the uploaded asset. Overrides the default backup settings of your product'
    /// environment.
    Backup(bool),
    /// A URL to redirect to after the upload is completed instead of returning the upload response. Signed upload
    /// result parameters are added to the callback URL. This parameter is ignored for XHR (Ajax XMLHttpRequest) or
    /// JavaScript Fetch API upload requests.
    ///
    /// NOTE: This parameter is relevant for direct uploads from a form in the browser. It is automatically set if you
    /// perform direct upload from the browser using Cloudinary's SDKs and the jQuery plugin.
    Callback(Url),
    /// Allows you to modify upload parameters by specifying custom logic with JavaScript. This can be useful for
    /// conditionally adding tags, contextual metadata, structured metadata or eager transformations depending on
    /// specific criteria of the uploaded file. For more details see
    /// [Evaluating and modifying upload parameters](https://cloudinary.com/documentation/upload_parameters#evaluating_and_modifying_upload_parameters).
    Eval(String),
    /// Allows you to update an asset by specifying custom logic with JavaScript that is executed after the upload to
    /// Cloudinary is completed successfully. This can be useful for conditionally adding tags, contextual metadata,
    /// and structured metadata, depending on the results of using the detection parameter on upload. For more details
    /// see [On Success update script](https://cloudinary.com/documentation/upload_parameters#on_success_update_script).
    OnSuccess(String),
    /// An HTTP header or a list of headers lines for adding as response HTTP headers when delivering the asset to your
    /// users. Supported headers: Link, Authorization, X-Robots-Tag.
    ///
    /// For example: X-Robots-Tag: noindex.
    Headers(Vec<AllowedHeaders>),
    /// Whether to invalidate CDN cached copies of a previously uploaded asset (and all transformed versions that share
    /// the same public ID).
    ///
    /// Default: false.
    ///
    /// It usually takes between a few seconds and a few minutes for the invalidation to fully propagate through the
    /// CDN. There are also a number of other
    /// [important considerations](https://cloudinary.com/documentation/invalidate_cached_media_assets_on_the_cdn)
    /// when using the invalidate functionality.
    Invalidate(bool),
    /// To request multiple moderations in a single API call:
    ///
    /// Send the desired list of moderations as a pipe-separated string with manual moderation, if relevant, being
    /// last.
    ///
    /// For example: aws_rek|duplicate:0|perception_point|manual
    ///
    /// NOTE: Rejected assets are automatically invalidated on the CDN within approximately ten minutes.
    ///
    /// (Asynchronous)
    Moderation(HashSet<Moderation>),
    /// An HTTP or HTTPS URL to receive the upload response (a webhook) when the upload or any requested asynchronous
    /// action is completed. If not specified, the response is sent to the Notification URL (if defined) in the Webhook
    /// Notifications settings of your Cloudinary Console.
    NotificationUrl(Url),
    /// Tells Cloudinary to upload assets from remote URLs through the given proxy.
    ///
    /// Format: https://hostname:port.
    Proxy(Url),
    /// Whether to return a deletion token in the upload response. The token can be used to delete the uploaded asset
    /// within 10 minutes using an unauthenticated API request.
    ///
    /// Default: false
    ReturnDeleteToken(bool),
}

impl OptionalParameters {
    pub fn get_pair(&self) -> (String, String) {
        match self {
            OptionalParameters::PublicId(s) => ("public_id".to_string(), s.to_string()),
            OptionalParameters::PublicIdPrefix(s) => {
                ("public_id_prefix".to_string(), s.to_string())
            }
            OptionalParameters::DisplayName(s) => ("display_name".to_string(), s.to_string()),
            OptionalParameters::AssetFolder(s) => ("asset_folder".to_string(), s.to_string()),
            OptionalParameters::UseAssetFolderAsPublicIdPrefix(b) => (
                "use_asset_folder_as_public_id_prefix".to_string(),
                b.to_string(),
            ),
            OptionalParameters::Folder(s) => ("folder".to_string(), s.to_string()),
            OptionalParameters::UseFileName(b) => ("use_filename".to_string(), b.to_string()),
            OptionalParameters::UseFilenameAsDisplayName(b) => {
                ("use_filename_as_display_name".to_string(), b.to_string())
            }
            OptionalParameters::UniqueFilename(b) => ("unique_filename".to_string(), b.to_string()),
            OptionalParameters::FilenameOverride(s) => {
                ("filename_override".to_string(), s.to_string())
            }
            OptionalParameters::ResourceType(s) => ("resource_type".to_string(), s.to_string()),
            OptionalParameters::Type(e) => ("type".to_string(), e.to_string()),
            OptionalParameters::AccessControl(s) => ("access_control".to_string(), s.to_string()),
            OptionalParameters::AccessMode(s) => ("access_mode".to_string(), s.to_string()),
            OptionalParameters::DiscardOriginalFilename(b) => {
                ("discard_original_filename".to_string(), b.to_string())
            }
            OptionalParameters::Overwrite(b) => ("overwrite".to_string(), b.to_string()),
            OptionalParameters::Tags(tags) => ("tags".to_string(), tags.iter().join(",")),
            OptionalParameters::Context(ctx) => (
                "context".to_string(),
                ctx.iter().map(|(k, v)| format!("{k}={v}")).join("|"),
            ),
            OptionalParameters::Metadata(metadata) => (
                "metadata".to_string(),
                metadata.iter().map(|(k, v)| format!("{k}={v}")).join("|"),
            ),
            OptionalParameters::ClearInvalid(clear_invalid) => {
                ("clear_invalid".to_string(), clear_invalid.to_string())
            }
            OptionalParameters::Colors(b) => ("colors".to_string(), b.to_string()),
            OptionalParameters::Faces(b) => ("faces".to_string(), b.to_string()),
            OptionalParameters::QualityAnalysis(b) => {
                ("quality_analysis".to_string(), b.to_string())
            }
            OptionalParameters::AccessibilityAnalysis(b) => {
                ("accessibility_analysis".to_string(), b.to_string())
            }
            OptionalParameters::CinemagraphAnalysis(b) => {
                ("cinemagraph_analysis".to_string(), b.to_string())
            }
            OptionalParameters::MediaMetadata(b) => ("media_metadata".to_string(), b.to_string()),
            OptionalParameters::Phash(b) => ("phash".to_string(), b.to_string()),
            OptionalParameters::ResponsiveBreakpoints(value) => (
                "responsive_breakpoints".to_string(),
                format!(
                    "['{}']",
                    value
                        .iter()
                        .map(|breakpoint| serde_json::to_string(breakpoint).unwrap())
                        .join("', '"),
                ),
            ),
            OptionalParameters::AutoTagging(f) => ("auto_tagging".to_string(), f.to_string()),
            OptionalParameters::Categorization(e) => ("categorization".to_string(), e.to_string()),
            OptionalParameters::Detection(s) => ("detection".to_string(), s.to_string()),
            OptionalParameters::AutoChaptering(b) => ("auto_chaptering".to_string(), b.to_string()),
            OptionalParameters::AutoTranscription(b) => {
                ("auto_transcription".to_string(), b.to_string())
            }
            OptionalParameters::Ocr => ("ocr".to_string(), "adv_ocr".to_string()),
            OptionalParameters::VisualSearch(b) => ("visual_search".to_string(), b.to_string()),
            OptionalParameters::Eager(vec) => (
                "eager".to_string(),
                vec.iter().map(|t| t.to_string()).join("|"),
            ),
            OptionalParameters::EagerAsync(b) => ("eager_async".to_string(), b.to_string()),
            OptionalParameters::EagerNotificationUrl(url) => {
                ("eager_notification_url".to_string(), url.to_string())
            }
            OptionalParameters::Transformation(vec) => (
                "transformation".to_string(),
                vec.iter().map(|t| t.to_string()).join("/"),
            ),
            OptionalParameters::Format(s) => ("format".to_string(), s.to_string()),
            OptionalParameters::CustomCoordinates(t) => {
                ("custom_coordinates".to_string(), t.iter().join(","))
            }
            OptionalParameters::Regions(hash_map) => (
                "regions".to_string(),
                serde_json::to_string(hash_map).expect("regions to be JSON serializable"),
            ),
            OptionalParameters::FaceCoordinates(vec) => (
                "face_coordinates".to_string(),
                vec.iter().map(|shape| shape.iter().join(",")).join("|"),
            ),
            OptionalParameters::AllowedFormats(vec) => {
                ("allowed_formats".to_string(), vec.join(","))
            }
            OptionalParameters::Async(b) => ("async".to_string(), b.to_string()),
            OptionalParameters::Backup(b) => ("backup".to_string(), b.to_string()),
            OptionalParameters::Callback(url) => ("callback".to_string(), url.to_string()),
            OptionalParameters::Eval(s) => ("eval".to_string(), s.to_string()),
            OptionalParameters::OnSuccess(s) => ("on_success".to_string(), s.to_string()),
            OptionalParameters::Headers(allowed_headers) => (
                "headers".to_string(),
                allowed_headers.iter().map(|h| h.to_string()).join("\n"),
            ),
            OptionalParameters::Invalidate(b) => ("invalidate".to_string(), b.to_string()),
            OptionalParameters::Moderation(hash_set) => {
                ("moderation".to_string(), hash_set.iter().join("|"))
            }
            OptionalParameters::NotificationUrl(url) => {
                ("notification_url".to_string(), url.to_string())
            }
            OptionalParameters::Proxy(url) => ("proxy".to_string(), url.to_string()),
            OptionalParameters::ReturnDeleteToken(b) => {
                ("return_delete_token".to_string(), b.to_string())
            }
            OptionalParameters::BackgroundRemoval(background_removal) => (
                "background_removal".to_string(),
                background_removal.to_string(),
            ),
            OptionalParameters::RawConvert(raw_convert) => {
                ("raw_convert".to_string(), raw_convert.to_string())
            }
        }
    }
}

impl fmt::Display for OptionalParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (k, v) = self.get_pair();
        write!(f, "{k}={v}")
    }
}

impl Hash for OptionalParameters {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_pair().0.hash(state);
    }
}

impl PartialEq for OptionalParameters {
    fn eq(&self, other: &Self) -> bool {
        self.get_pair().0 == other.get_pair().0
    }
}

impl Eq for OptionalParameters {}

impl PartialOrd for OptionalParameters {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptionalParameters {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_pair().0.cmp(&other.get_pair().0)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        str::FromStr,
    };

    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use url::Url;

    use crate::{
        transformation::{crop_mode::CropMode, pad_mode::PadMode, Transformations},
        upload::{
            access_mode::AccessModes, allowed_headers::AllowedHeaders,
            background_removal::BackgroundRemoval, categorizations::Categorizations,
            delivery_type::DeliveryType, moderation::Moderation, options::OptionalParameters,
            raw_convert::RawConvert, resource_type::ResourceTypes,
            responsive_breakpoints::ResponsiveBreakpoints,
        },
    };

    #[test]
    fn public_id() {
        assert_eq!(
            OptionalParameters::PublicId("id".to_string()).get_pair(),
            ("public_id".to_string(), "id".to_string())
        )
    }

    #[test]
    fn public_id_prefix() {
        assert_eq!(
            OptionalParameters::PublicIdPrefix("prefix".to_string()).get_pair(),
            ("public_id_prefix".to_string(), "prefix".to_string())
        )
    }

    #[test]
    fn display_name() {
        assert_eq!(
            OptionalParameters::DisplayName("name".to_string()).get_pair(),
            ("display_name".to_string(), "name".to_string())
        )
    }

    #[test]
    fn asset_folder() {
        assert_eq!(
            OptionalParameters::AssetFolder("folder".to_string()).get_pair(),
            ("asset_folder".to_string(), "folder".to_string())
        )
    }

    #[test]
    fn use_asset_folder_as_public_id_prefix() {
        assert_eq!(
            OptionalParameters::UseAssetFolderAsPublicIdPrefix(true).get_pair(),
            (
                "use_asset_folder_as_public_id_prefix".to_string(),
                "true".to_string()
            )
        )
    }

    #[test]
    fn folder() {
        assert_eq!(
            OptionalParameters::Folder("value".to_string()).get_pair(),
            ("folder".to_string(), "value".to_string())
        )
    }

    #[test]
    fn use_filename() {
        assert_eq!(
            OptionalParameters::UseFileName(false).get_pair(),
            ("use_filename".to_string(), "false".to_string())
        )
    }

    #[test]
    fn use_filename_as_display_name() {
        assert_eq!(
            OptionalParameters::UseFilenameAsDisplayName(false).get_pair(),
            (
                "use_filename_as_display_name".to_string(),
                "false".to_string()
            )
        )
    }

    #[test]
    fn unique_filename() {
        assert_eq!(
            OptionalParameters::UniqueFilename(true).get_pair(),
            ("unique_filename".to_string(), "true".to_string())
        )
    }

    #[test]
    fn filename_override() {
        assert_eq!(
            OptionalParameters::FilenameOverride("value".to_string()).get_pair(),
            ("filename_override".to_string(), "value".to_string())
        )
    }

    #[test]
    fn resource_type() {
        assert_eq!(
            OptionalParameters::ResourceType(ResourceTypes::Raw).get_pair(),
            ("resource_type".to_string(), "raw".to_string())
        );
        assert_eq!(
            OptionalParameters::ResourceType(ResourceTypes::Auto).get_pair(),
            ("resource_type".to_string(), "auto".to_string())
        );
        assert_eq!(
            OptionalParameters::ResourceType(ResourceTypes::Image).get_pair(),
            ("resource_type".to_string(), "image".to_string())
        );
        assert_eq!(
            OptionalParameters::ResourceType(ResourceTypes::Video).get_pair(),
            ("resource_type".to_string(), "video".to_string())
        );
    }

    #[test]
    fn r#type() {
        assert_eq!(
            OptionalParameters::Type(DeliveryType::Authenticated).get_pair(),
            ("type".to_string(), "authenticated".to_string())
        );
        assert_eq!(
            OptionalParameters::Type(DeliveryType::Private).get_pair(),
            ("type".to_string(), "private".to_string())
        );
        assert_eq!(
            OptionalParameters::Type(DeliveryType::Upload).get_pair(),
            ("type".to_string(), "upload".to_string())
        );
    }

    #[test]
    fn access_control() {
        assert_eq!(
            OptionalParameters::AccessControl("control".to_string()).get_pair(),
            ("access_control".to_string(), "control".to_string())
        );
    }

    #[test]
    fn access_mode() {
        assert_eq!(
            OptionalParameters::AccessMode(AccessModes::Authenticated).get_pair(),
            ("access_mode".to_string(), "authenticated".to_string())
        );
        assert_eq!(
            OptionalParameters::AccessMode(AccessModes::Public).get_pair(),
            ("access_mode".to_string(), "public".to_string())
        );
    }

    #[test]
    fn discard_original_filename() {
        assert_eq!(
            OptionalParameters::DiscardOriginalFilename(true).get_pair(),
            ("discard_original_filename".to_string(), "true".to_string())
        );
    }

    #[test]
    fn overwrite() {
        assert_eq!(
            OptionalParameters::Overwrite(true).get_pair(),
            ("overwrite".to_string(), "true".to_string())
        );
    }

    #[test]
    fn tags() {
        let tags = HashSet::from(["one".to_string(), "two".to_string()]);
        let str = tags.iter().join(",");
        assert_eq!(
            OptionalParameters::Tags(tags).get_pair(),
            ("tags".to_string(), str)
        );
    }

    #[test]
    fn context() {
        let ctx = HashMap::from([
            ("one".to_string(), "two".to_string()),
            ("1".to_string(), "2".to_string()),
        ]);
        let str = ctx.iter().map(|(k, v)| format!("{k}={v}")).join("|");
        assert_eq!(
            OptionalParameters::Context(ctx).get_pair(),
            ("context".to_string(), str)
        );
    }

    #[test]
    fn metadata() {
        let ctx = HashMap::from([
            ("one".to_string(), "two".to_string()),
            ("1".to_string(), "2".to_string()),
        ]);
        let str = ctx.iter().map(|(k, v)| format!("{k}={v}")).join("|");
        assert_eq!(
            OptionalParameters::Metadata(ctx).get_pair(),
            ("metadata".to_string(), str)
        );
    }

    #[test]
    fn clear_invalid() {
        assert_eq!(
            OptionalParameters::ClearInvalid(true).get_pair(),
            ("clear_invalid".to_string(), "true".to_string())
        );
    }

    #[test]
    fn colors() {
        assert_eq!(
            OptionalParameters::Colors(true).get_pair(),
            ("colors".to_string(), "true".to_string())
        );
    }

    #[test]
    fn faces() {
        assert_eq!(
            OptionalParameters::Faces(true).get_pair(),
            ("faces".to_string(), "true".to_string())
        );
    }

    #[test]
    fn quality_analysis() {
        assert_eq!(
            OptionalParameters::QualityAnalysis(true).get_pair(),
            ("quality_analysis".to_string(), "true".to_string())
        );
    }

    #[test]
    fn accessibility_analysis() {
        assert_eq!(
            OptionalParameters::AccessibilityAnalysis(true).get_pair(),
            ("accessibility_analysis".to_string(), "true".to_string())
        );
    }

    #[test]
    fn cinemagraph_analysis() {
        assert_eq!(
            OptionalParameters::CinemagraphAnalysis(true).get_pair(),
            ("cinemagraph_analysis".to_string(), "true".to_string())
        );
    }

    #[test]
    fn media_metadata() {
        assert_eq!(
            OptionalParameters::MediaMetadata(true).get_pair(),
            ("media_metadata".to_string(), "true".to_string())
        );
    }

    #[test]
    fn phash() {
        assert_eq!(
            OptionalParameters::Phash(true).get_pair(),
            ("phash".to_string(), "true".to_string())
        );
    }

    #[test]
    fn responsive_breakpoints() {
        let breakpoints: Vec<ResponsiveBreakpoints> = vec![
            ResponsiveBreakpoints::new(true),
            ResponsiveBreakpoints {
                create_derived: false,
                format: Some(String::from("foo")),
                transformation: Some(String::from("tar")),
                max_width: Some(2),
                min_width: Some(1),
                bytes_step: Some(3),
                max_images: Some(4),
            },
        ];
        assert_eq!(
            OptionalParameters::ResponsiveBreakpoints(breakpoints).get_pair(),
            (
                "responsive_breakpoints".to_string(),
                "['{\"create_derived\":true}', '{\"create_derived\":false,\"format\":\"foo\",\"transformation\":\"tar\",\"max_width\":2,\"min_width\":1,\"bytes_step\":3,\"max_images\":4}']".to_string()
            )
        );
    }

    #[test]
    fn auto_tagging() {
        assert_eq!(
            OptionalParameters::AutoTagging(0.4).get_pair(),
            ("auto_tagging".to_string(), "0.4".to_string())
        );
    }

    #[test]
    fn categorization() {
        assert_eq!(
            OptionalParameters::Categorization(Categorizations::Imagga).get_pair(),
            ("categorization".to_string(), "imagga_tagging".to_string())
        );
    }

    #[test]
    fn detection() {
        assert_eq!(
            OptionalParameters::Detection("yolo".to_string()).get_pair(),
            ("detection".to_string(), "yolo".to_string())
        );
    }

    #[test]
    fn auto_chaptering() {
        assert_eq!(
            OptionalParameters::AutoChaptering(true).get_pair(),
            ("auto_chaptering".to_string(), "true".to_string())
        );
    }

    #[test]
    fn auto_transcription() {
        assert_eq!(
            OptionalParameters::AutoTranscription(false).get_pair(),
            ("auto_transcription".to_string(), "false".to_string())
        );
    }

    #[test]
    fn ocr() {
        assert_eq!(
            OptionalParameters::Ocr.get_pair(),
            ("ocr".to_string(), "adv_ocr".to_string())
        );
    }

    #[test]
    fn visual_search() {
        assert_eq!(
            OptionalParameters::VisualSearch(false).get_pair(),
            ("visual_search".to_string(), "false".to_string())
        );
    }

    #[test]
    fn eager() {
        let transformations: Vec<Transformations> = vec![
            Transformations::Crop(CropMode::Fill {
                width: 1,
                height: 2,
                gravity: None,
            }),
            Transformations::Pad(PadMode::Pad {
                width: 3,
                height: 4,
                background: None,
                gravity: None,
            }),
        ];
        assert_eq!(
            OptionalParameters::Eager(transformations).get_pair(),
            (
                "eager".to_string(),
                "c_fill,w_1,h_2|c_pad,w_3,h_4".to_string()
            )
        );
    }

    #[test]
    fn eager_async() {
        assert_eq!(
            OptionalParameters::EagerAsync(true).get_pair(),
            ("eager_async".to_string(), "true".to_string())
        );
    }

    #[test]
    fn eager_notification_url() {
        assert_eq!(
            OptionalParameters::EagerNotificationUrl(
                Url::from_str("localhost:3000").expect("to be url")
            )
            .get_pair(),
            (
                "eager_notification_url".to_string(),
                "localhost:3000".to_string()
            )
        );
    }

    #[test]
    fn transformation() {
        let transformations: Vec<Transformations> = vec![
            Transformations::Crop(CropMode::Fill {
                width: 1,
                height: 2,
                gravity: None,
            }),
            Transformations::Pad(PadMode::Pad {
                width: 3,
                height: 4,
                background: None,
                gravity: None,
            }),
        ];

        assert_eq!(
            OptionalParameters::Transformation(transformations).get_pair(),
            (
                "transformation".to_string(),
                "c_fill,w_1,h_2/c_pad,w_3,h_4".to_string()
            )
        );
    }

    #[test]
    fn format() {
        assert_eq!(
            OptionalParameters::Format("jpg".to_string()).get_pair(),
            ("format".to_string(), "jpg".to_string())
        );
    }

    #[test]
    fn custom_coordinates() {
        assert_eq!(
            OptionalParameters::CustomCoordinates([4, 3, 2, 1]).get_pair(),
            ("custom_coordinates".to_string(), "4,3,2,1".to_string())
        );
    }

    #[test]
    fn regions() {
        let data = HashMap::from([
            ("name".to_string(), vec![[1, 2], [3, 4]]),
            ("name2".to_string(), vec![[9, 8], [7, 6]]),
        ]);
        assert_eq!(
            OptionalParameters::Regions(data.clone()).get_pair(),
            (
                "regions".to_string(),
                serde_json::to_string(&data).expect("data to be serializable")
            )
        );
    }

    #[test]
    fn face_coordinates() {
        assert_eq!(
            OptionalParameters::FaceCoordinates(Vec::from([[1, 2, 3, 4], [9, 8, 7, 6]])).get_pair(),
            (
                "face_coordinates".to_string(),
                "1,2,3,4|9,8,7,6".to_string()
            )
        );
    }

    #[test]
    fn allowed_formats() {
        assert_eq!(
            OptionalParameters::AllowedFormats(Vec::from(["jpg".to_string(), "gif".to_string()]))
                .get_pair(),
            ("allowed_formats".to_string(), "jpg,gif".to_string())
        );
    }

    #[test]
    fn r#async() {
        assert_eq!(
            OptionalParameters::Async(true).get_pair(),
            ("async".to_string(), "true".to_string())
        );
    }

    #[test]
    fn backup() {
        assert_eq!(
            OptionalParameters::Backup(true).get_pair(),
            ("backup".to_string(), "true".to_string())
        );
    }

    #[test]
    fn callback() {
        assert_eq!(
            OptionalParameters::Callback(
                Url::from_str("localhost:3000").expect("to be a valid url")
            )
            .get_pair(),
            ("callback".to_string(), "localhost:3000".to_string())
        );
    }

    #[test]
    fn eval() {
        assert_eq!(
            OptionalParameters::Eval("Look ma I am Javascript".to_string()).get_pair(),
            ("eval".to_string(), "Look ma I am Javascript".to_string())
        );
    }

    #[test]
    fn on_success() {
        assert_eq!(
            OptionalParameters::OnSuccess("Look ma I am Javascript, again".to_string()).get_pair(),
            (
                "on_success".to_string(),
                "Look ma I am Javascript, again".to_string()
            )
        );
    }

    #[test]
    fn headers() {
        assert_eq!(
            OptionalParameters::Headers(Vec::from([AllowedHeaders::Link(
                "<localhost:3000>;".to_string()
            )]))
            .get_pair(),
            ("headers".to_string(), "Link: <localhost:3000>;".to_string())
        );
    }

    #[test]
    fn invalidate() {
        assert_eq!(
            OptionalParameters::Invalidate(true).get_pair(),
            ("invalidate".to_string(), "true".to_string())
        );
    }

    #[test]
    fn moderation() {
        let moderation = HashSet::from([Moderation::Duplicate(0.1), Moderation::Duplicate(0.2)]);
        assert_eq!(
            OptionalParameters::Moderation(moderation).get_pair(),
            ("moderation".to_string(), "duplicate:0.1".to_string())
        );
    }

    #[test]
    fn multiple_moderation() {
        let moderation = HashSet::from([Moderation::Duplicate(0.1), Moderation::Manual]);
        let str = moderation.iter().join("|");
        assert_eq!(
            OptionalParameters::Moderation(moderation).get_pair(),
            ("moderation".to_string(), str)
        );
    }

    #[test]
    fn notification_url() {
        assert_eq!(
            OptionalParameters::NotificationUrl(Url::from_str("localhost:3000").unwrap())
                .get_pair(),
            ("notification_url".to_string(), "localhost:3000".to_string())
        )
    }

    #[test]
    fn proxy() {
        assert_eq!(
            OptionalParameters::Proxy(Url::from_str("localhost:3000").unwrap()).get_pair(),
            ("proxy".to_string(), "localhost:3000".to_string())
        )
    }

    #[test]
    fn return_delete_token() {
        assert_eq!(
            OptionalParameters::ReturnDeleteToken(false).get_pair(),
            ("return_delete_token".to_string(), "false".to_string())
        )
    }

    #[test]
    fn background_removal() {
        assert_eq!(
            OptionalParameters::BackgroundRemoval(BackgroundRemoval::Pixelz).get_pair(),
            ("background_removal".to_string(), "pixelz".to_string())
        )
    }

    #[test]
    fn raw_convert() {
        assert_eq!(
            OptionalParameters::RawConvert(RawConvert::ExtractText).get_pair(),
            ("raw_convert".to_string(), "extract_text".to_string())
        )
    }
}
