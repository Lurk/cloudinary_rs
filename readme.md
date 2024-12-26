# cloudinary
<!-- cargo-rdme start -->

[![codecov](https://codecov.io/gh/Lurk/cloudinary_rs/branch/main/graph/badge.svg?token=K8H5DLTSX4)](https://codecov.io/gh/Lurk/cloudinary_rs)
[![crates.io](https://img.shields.io/crates/v/cloudinary.svg)](https://crates.io/crates/cloudinary)
[![Released API docs](https://docs.rs/cloudinary/badge.svg)](https://docs.rs/cloudinary)

At the moment, there is only half-backed upload and transformation functionality, but if you need more, please let
me know.

## Upload an image

Upload can be done from different sources:

- local file
- remote file
- data url [rfc2397](https://datatracker.ietf.org/doc/html/rfc2397)

### Local file

```rust
use std::collections::BTreeSet;
use cloudinary::upload::{Source, Upload, options::OptionalParameters};

let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let options = BTreeSet::from([OptionalParameters::PublicId("file.jpg".to_string())]);
let result = upload.image(Source::Path("./image.jpg".into()), &options);
```

### Remote file

```rust
use std::collections::BTreeSet;
use cloudinary::upload::{Source, Upload, options::OptionalParameters};

let image_url = "https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png";
let options = BTreeSet::from([OptionalParameters::PublicId("1x1.png".to_string())]);
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.image(Source::Path("./image.jpg".into()), &options);
```

### Data url

```rust
use std::collections::BTreeSet;
use cloudinary::upload::{Source, Upload, options::OptionalParameters};

let data_url = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==";
let options = BTreeSet::from([OptionalParameters::PublicId("1x1.png".to_string())]);
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.image(Source::DataUrl(data_url.to_string()), &options);
```

## Destroy an asset by publicID
```rust
use cloudinary::upload::Upload;
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.destroy("publicID");
```

## Transform an image

Currently supported transformations:
* Resize
* Crop
* Pad

### Resizing an image:

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

### Cropping an image:

```rust
use cloudinary::transformation::{
   Transformations::Crop, crop_mode::CropMode::FillByWidth, Image, aspect_ratio::AspectRatio
};

let image = Image::new("test".into(), "path/name.png".into())
    .add_transformation(Crop(FillByWidth{ width:100, ar: None, gravity: None}));

assert_eq!(
    image.to_string(),
    "https://res.cloudinary.com/test/image/upload/c_fill,w_100/path/name.png"
);
```

### Padding an image:

```rust
use cloudinary::transformation::{
  Transformations::Pad, pad_mode::PadMode::PadByWidth, Image, aspect_ratio::AspectRatio
};

let image = Image::new("test".into(), "path/name.png".into())
    .add_transformation(Pad(PadByWidth{ width:100, ar: None, background: None, gravity: None}));
assert_eq!(
   image.to_string(),
   "https://res.cloudinary.com/test/image/upload/c_pad,w_100/path/name.png"
);
```

## Get Image from URL

Unofficial api. This is not supported by Cloudinary, and can break at any time.
Officially you should use public_id that you get from upload.

[Support](https://support.cloudinary.com/hc/en-us/community/posts/360006941639-How-to-programmatically-retrieve-public-id-from-URL-)

```rust
use cloudinary::transformation::Image;
use url::Url;
let image = Image::try_from(
    Url::parse("https://res.cloudinary.com/test/image/upload/path/name.png").unwrap()
).unwrap();
assert_eq!(image.to_string(), "https://res.cloudinary.com/test/image/upload/path/name.png");
```

## Get a list of all assets with a given tag
```rust
use cloudinary::tags::get_tags;
let tags = get_tags("cloud_name".into(), "tag_name".into()).await;

```


## Development

Due to differences in default upload result shape in different accounts, two sets
of credentials must be present in `.env` for tests to succeed.

```sh
CLOUDINARY_API_SECRET=***
CLOUDINARY_API_KEY=***
CLOUDINARY_CLOUD_NAME=***

CLOUDINARY_API_SECRET_1=***
CLOUDINARY_API_KEY_1=***
CLOUDINARY_CLOUD_NAME_1=***
```

## Minimum supported Rust version

The minimum supported Rust version for this crate is 1.65

<!-- cargo-rdme end -->
[![codecov](https://codecov.io/gh/Lurk/cloudinary_rs/branch/main/graph/badge.svg?token=K8H5DLTSX4)](https://codecov.io/gh/Lurk/cloudinary_rs)
[![crates.io](https://img.shields.io/crates/v/cloudinary.svg)](https://crates.io/crates/cloudinary)
[![Released API docs](https://docs.rs/cloudinary/badge.svg)](https://docs.rs/cloudinary)

At the moment, there is only half-backed upload and transformation functionality, but if you need more, please let
me know.

## Upload an image

Upload can be done from different sources:

- local file
- remote file
- data url [rfc2397](https://datatracker.ietf.org/doc/html/rfc2397)

### Local file

```rust
use cloudinary::upload::{UploadOptions, Source, Upload};
let options = UploadOptions::new().set_public_id("file.jpg".to_string());
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.image(Source::Path("./image.jpg".into()), &options);
```

### Remote file

```rust
use cloudinary::upload::{UploadOptions, Source, Upload};
let image_url = "https://upload.wikimedia.org/wikipedia/commons/c/ca/1x1.png";
let options = UploadOptions::new().set_public_id("1x1.png".to_string());
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.image(Source::Url(image_url.try_into().unwrap()), &options);
```

### Data url

```rust
use cloudinary::upload::{UploadOptions, Source, Upload};
let data_url = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==";
let options = UploadOptions::new().set_public_id("1x1.png".to_string());
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.image(Source::DataUrl(data_url.into()), &options);
```

## Destroy an asset by publicID
```rust
use cloudinary::upload::Upload;
let upload = Upload::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = upload.destroy("publicID");
```

## Transform an image

Currently supported transformations:
* Resize
* Crop
* Pad

### Resizing an image:

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

### Cropping an image:

```rust
use cloudinary::transformation::{
   Transformations::Crop, crop_mode::CropMode::FillByWidth, Image, aspect_ratio::AspectRatio
};

let image = Image::new("test".into(), "path/name.png".into())
    .add_transformation(Crop(FillByWidth{ width:100, ar: None, gravity: None}));

assert_eq!(
    image.to_string(),
    "https://res.cloudinary.com/test/image/upload/c_fill,w_100/path/name.png"
);
```

### Padding an image:

```rust
use cloudinary::transformation::{
  Transformations::Pad, pad_mode::PadMode::PadByWidth, Image, aspect_ratio::AspectRatio
};

let image = Image::new("test".into(), "path/name.png".into())
    .add_transformation(Pad(PadByWidth{ width:100, ar: None, background: None, gravity: None}));
assert_eq!(
   image.to_string(),
   "https://res.cloudinary.com/test/image/upload/c_pad,w_100/path/name.png"
);
```

## Get Image from URL

Unofficial api. This is not supported by Cloudinary, and can break at any time.
Officially you should use public_id that you get from upload.

[Support](https://support.cloudinary.com/hc/en-us/community/posts/360006941639-How-to-programmatically-retrieve-public-id-from-URL-)

```rust
use cloudinary::transformation::Image;
use url::Url;
let image = Image::try_from(
    Url::parse("https://res.cloudinary.com/test/image/upload/path/name.png").unwrap()
).unwrap();
assert_eq!(image.to_string(), "https://res.cloudinary.com/test/image/upload/path/name.png");
```

## Get a list of all assets with a given tag
```rust
use cloudinary::tags::get_tags;
let tags = get_tags("cloud_name".into(), "tag_name".into()).await;

```


## Development

Due to differences in default upload result shape in different accounts, two sets
of credentials must be present in `.env` for tests to succeed.

```sh
CLOUDINARY_API_SECRET=***
CLOUDINARY_API_KEY=***
CLOUDINARY_CLOUD_NAME=***

CLOUDINARY_API_SECRET_1=***
CLOUDINARY_API_KEY_1=***
CLOUDINARY_CLOUD_NAME_1=***
```

## Minimum supported Rust version

The minimum supported Rust version for this crate is 1.65


License: MIT OR Apache-2.0
