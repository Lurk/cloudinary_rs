use dotenv::dotenv;
use std::env::var;

use crate::{
    upload::UploadOptions,
    upload::{
        result::UploadResult::{Error, Success},
        Source, Upload,
    },
};

fn env() -> (String, String, String) {
    dotenv().ok();
    let api_key = var("CLOUDINARY_API_KEY").expect("environment variables not set");
    let cloud_name = var("CLOUDINARY_CLOUD_NAME").expect("environment variables not set");
    let api_secret = var("CLOUDINARY_API_SECRET").expect("environment variables not set");

    (api_key, cloud_name, api_secret)
}

#[tokio::test]
async fn test_image_upload_from_base64() {
    let (api_key, cloud_name, api_secret) = env();
    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_base64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==";
    let public_id = "image_upload_from_base64";

    let options = UploadOptions::new()
        .set_public_id(public_id.into())
        .set_overwrite(true);
    let res = cloudinary
        .image(Source::DataUrl(image_base64.into()), &options)
        .await
        .unwrap();

    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
    }
}

#[tokio::test]
async fn test_image_upload_from_url() {
    let (api_key, cloud_name, api_secret) = env();
    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_url = "https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png";
    let public_id = "image_upload_from_url";

    let options = UploadOptions::new()
        .set_public_id(public_id.into())
        .set_overwrite(true);
    let res = cloudinary
        .image(Source::Url(image_url.try_into().unwrap()), &options)
        .await
        .unwrap();

    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
    }
}

#[tokio::test]
async fn test_image_upload_from_path() {
    let (api_key, cloud_name, api_secret) = env();
    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_path = "./assets/1x1.png";
    let public_id = "image_upload_from_path";

    let options = UploadOptions::new()
        .set_public_id(public_id.into())
        .set_overwrite(true);
    let res = cloudinary
        .image(Source::Path(image_path.into()), &options)
        .await
        .unwrap();

    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
    }
}
