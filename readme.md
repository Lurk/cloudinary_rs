# cloudinary

[![codecov](https://codecov.io/gh/Lurk/cloudinary_rs/branch/main/graph/badge.svg?token=K8H5DLTSX4)](https://codecov.io/gh/Lurk/cloudinary_rs)
[![crates.io](https://img.shields.io/crates/v/cloudinary.svg)](https://crates.io/crates/cloudinary)
[![Released API docs](https://docs.rs/cloudinary/badge.svg)](https://docs.rs/cloudinary)

At the moment, there is only half-backed upload and transformation functionality, but if you need more, please let
me know.

## Upload an image

```rust
use cloudinary::{Source, Cloudinary};
use cloudinary::upload::{UploadOptions};
let options = UploadOptions::new().set_public_id("file.jpg".to_string());
let cloudinary = Cloudinary::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = cloudinary.upload_image(Source::Path("./image.jpg".into()), &options);
```

## Transform an image

```rust
use cloudinary::transformation::{
    Transformations::Resize, resize_mode::ResizeMode::ScaleByWidth, Image, aspect_ratio::AspectRatio
};

let image = Image::new("test".into(), "path/name.png".into())
    .add_transformation(Resize(ScaleByWidth{ width:100, ar: None, liquid:None}));
assert_eq!(
    image.to_string(),
    "https://res.cloudinary.com/test/image/upload/c_scale,w_100/path/name.png"
);
```
## Minimum supported Rust version

The minimum supported Rust version for this crate is 1.65


License: MIT OR Apache-2.0
