use dotenv::dotenv;
use pretty_assertions::assert_eq;
use std::env::var;

use crate::{
    upload::UploadOptions,
    upload::{
        result::UploadResult::{Error, Response, ResponseWithImageMetadata},
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
        Response(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
        _ => {
            panic!("Since old account was used, only Response or Error variant is expected")
        }
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
        Response(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
        _ => {
            panic!("Since old account was used, only Response or Error variant is expected")
        }
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
        Response(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
        _ => {
            panic!("Since old account was used, only Response or Error variant is expected")
        }
    }
}

#[tokio::test]
async fn test_destroy_non_existing_asset() {
    let (api_key, cloud_name, api_secret) = env();
    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let public_id = "random-1239290r29-does-it-exists-3we97pcsdlncdsa";

    let res = cloudinary.destroy(public_id).await.unwrap();

    assert_eq!(res.result, "not found")
}

#[tokio::test]
async fn test_destroy_existing_asset() {
    let (api_key, cloud_name, api_secret) = env();
    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_path = "./assets/1x1.png";
    let public_id = format!("asset_to_destroy_{}", chrono::Utc::now().timestamp_micros());

    let options = UploadOptions::new()
        .set_public_id(public_id.clone())
        .set_overwrite(true);
    let res = cloudinary
        .image(Source::Path(image_path.into()), &options)
        .await
        .unwrap();

    match res {
        Response(_) => {
            let res = cloudinary.destroy(public_id).await.unwrap();
            assert_eq!(res.result, "ok")
        }
        Error(err) => panic!("{}", err.error.message),
        _ => {
            panic!("Since old account was used, only Response or Error variant is expected")
        }
    }
}

#[tokio::test]
async fn test_image_upload_from() {
    dotenv().ok();
    let api_key = var("CLOUDINARY_API_KEY_1").expect("environment variables not set");
    let cloud_name = var("CLOUDINARY_CLOUD_NAME_1").expect("environment variables not set");
    let api_secret = var("CLOUDINARY_API_SECRET_1").expect("environment variables not set");

    let cloudinary = Upload::new(api_key, cloud_name, api_secret);
    let image_base64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==";
    let public_id = "image_upload_from_base64";

    let options = UploadOptions::new()
        .set_image_metadata(true)
        .set_public_id(public_id.into())
        .set_overwrite(true);
    let res = cloudinary
        .image(Source::DataUrl(image_base64.into()), &options)
        .await
        .unwrap();

    match res {
        Error(err) => panic!("{}", err.error.message),
        ResponseWithImageMetadata(img) => assert_eq!(img.public_id, public_id),
        _ => panic!("Since new account was used, only ResponseWithImageMetadata or Error variant is expected"),
    }
}
