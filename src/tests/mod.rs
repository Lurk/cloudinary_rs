use dotenv::dotenv;
use std::env::var;

use crate::{
    upload::UploadOptions,
    upload::{
        result::UploadResult::{Error, Success},
        Source, Upload,
    },
};

#[tokio::test]
async fn test_image_upload_from_url() {
    dotenv().ok();
    let api_secret = var("CLOUDINARY_API_SECRET").expect("enviroment variables not set");
    let api_key = var("CLOUDINARY_API_KEY").expect("enviroment variables not set");
    let cloud_name = var("CLOUDINARY_CLOUD_NAME").expect("enviroment variables not set");

    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_url = "https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png";
    let public_id = "image_upload_from_url";

    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
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
    dotenv().ok();
    let api_secret = var("CLOUDINARY_API_SECRET").expect("enviroment variables not set");
    let api_key = var("CLOUDINARY_API_KEY").expect("enviroment variables not set");
    let cloud_name = var("CLOUDINARY_CLOUD_NAME").expect("enviroment variables not set");

    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_path = "./assets/1x1.png";
    let public_id = "image_upload_from_path";

    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
        .set_overwrite(true);
    let res = cloudinary
        .image(Source::Path(image_path.try_into().unwrap()), &options)
        .await
        .unwrap();

    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
    }
}
