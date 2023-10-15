# Cloudinary Rust API

[![codecov](https://codecov.io/gh/Lurk/cloudinary_rs/branch/main/graph/badge.svg?token=K8H5DLTSX4)](https://codecov.io/gh/Lurk/cloudinary_rs)
[![crates.io](https://img.shields.io/crates/v/cloudinary.svg)](https://crates.io/crates/cloudinary)
[![Released API docs](https://docs.rs/cloudinary/badge.svg)](https://docs.rs/cloudinary)

At the moment, there is only half-backed upload functionality, but if you need more, please let me know.

## Usage

```rust
use cloudinary::{Source, Cloudinary};
use cloudinary::upload::{UploadOptions};
let options = UploadOptions::new().set_public_id("file.jpg".to_string());
let cloudinary = Cloudinary::new("api_key".to_string(), "cloud_name".to_string(), "api_secret".to_string() );
let result = cloudinary.upload_image(Source::Path("./image.jpg".into()), &options);
```
