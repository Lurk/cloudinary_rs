use dotenv::dotenv;
use std::env::var;

use crate::{
    upload::result::UploadResult::{Error, Success},
    upload::{resource_type::ResourceTypes, UploadOptions},
    Cloudinary, Source,
};

struct CloudinaryVariable {
    pub api_secret: String,
    pub api_key: String,
    pub cloud_name: String,
}

fn load_env() -> CloudinaryVariable {
    dotenv().ok();
    let api_secret = var("CLOUDINARY_API_SECRET").expect("enviroment variables not set");
    let api_key = var("CLOUDINARY_API_KEY").expect("enviroment variables not set");
    let cloud_name = var("CLOUDINARY_CLOUD_NAME").expect("enviroment variables not set");
    CloudinaryVariable {
        api_secret,
        api_key,
        cloud_name,
    }
}

#[tokio::test]
async fn test_raw_upload() {
    let env_vars = load_env();
    let cloudinary = Cloudinary::new(env_vars.api_key, env_vars.cloud_name, env_vars.api_secret);

    let audio_url = "https://commons.wikimedia.org/wiki/File:Panama_National_Anthem.ogg";
    let public_id = "audio_from_url.ogg";
    let options = UploadOptions::new()
        .set_resource_type(ResourceTypes::Raw)
        .set_public_id(String::from(public_id))
        .set_overwrite(true);
    let res = cloudinary
        .upload(Source::Url(audio_url.try_into().unwrap()), &options)
        .await
        .unwrap();

    match res {
        Success(audio) => assert_eq!(audio.public_id, public_id.to_string()),
        Error(err) => panic!("{}", err.error.message),
    }
}

#[tokio::test]
async fn test_image_upload_from_url() {
    let env_vars = load_env();
    let cloudinary = Cloudinary::new(env_vars.api_key, env_vars.cloud_name, env_vars.api_secret);

    let image_url = "https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png";
    let public_id = "image_upload_from_url";

    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
        .set_overwrite(true);
    let res = cloudinary
        .upload_image(Source::Url(image_url.try_into().unwrap()), &options)
        .await
        .unwrap();

    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
    }
}

#[tokio::test]
async fn test_image_upload_from_path() {
    let env_vars = load_env();
    let cloudinary = Cloudinary::new(env_vars.api_key, env_vars.cloud_name, env_vars.api_secret);
    let image_path = "./assets/1x1.png";
    let public_id = "image_upload_from_path";

    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
        .set_overwrite(true);
    let res = cloudinary
        .upload_image(Source::Path(image_path.try_into().unwrap()), &options)
        .await
        .unwrap();

    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message),
    }
}
