use dotenv::dotenv;
use std::env::var;

use crate::{Cloudinary, upload::UploadOptions, UploadResult::{Success, Error}};

#[tokio::test]
async fn test_image_upload_from_url() {
    dotenv().ok();
    let api_secret = var("CLOUDINARY_API_SECRET").expect("enviroment variables not set");
    let api_key = var("CLOUDINARY_API_KEY").expect("enviroment variables not set");
    let cloud_name = var("CLOUDINARY_CLOUD_NAME").expect("enviroment variables not set");

    let cloudinary = Cloudinary::new(api_key, cloud_name, api_secret);
    let image_url = "https://www.pngall.com/wp-content/uploads/14/Gojo-PNG-Free-Image.png";
    let public_id = "satoru_freaking_gojo";
    
    let options = UploadOptions::new()
        .set_public_id(String::from(public_id))
        .set_overwrite(true);
    let res = cloudinary.upload_image_from_url(String::from(image_url), &options).await.unwrap();
    
    match res {
        Success(img) => assert_eq!(img.public_id, public_id),
        Error(err) => panic!("{}", err.error.message)
    }
}
