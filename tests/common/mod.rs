#![allow(dead_code)]

use imageio::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn work_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-output")
        .join(name);
    fs::create_dir_all(&dir).expect("create test output dir");
    dir
}

pub fn sample_image() -> DecodedImage {
    DecodedImage {
        width: 2,
        height: 2,
        bgra: vec![
            0, 0, 255, 255, // red
            0, 255, 0, 255, // green
            255, 0, 0, 255, // blue
            255, 255, 255, 255,
        ],
    }
}

pub fn sample_png_bytes() -> Vec<u8> {
    let image = sample_image();
    encode_bgra_to_bytes(&image.bgra, image.width, image.height, ImageFormat::Png)
        .expect("encode sample png")
}

pub fn write_sample_png(path: &Path) {
    fs::write(path, sample_png_bytes()).expect("write sample png");
}

pub fn animated_gif_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("animated.gif")
}

pub fn sample_metadata() -> Metadata {
    let mut metadata = MutableMetadata::new().expect("create metadata");
    metadata
        .register_namespace_for_prefix("http://ns.adobe.com/xap/1.0/", "xmp")
        .expect("register namespace");
    let tag = MetadataTag::new_string(
        "http://ns.adobe.com/xap/1.0/",
        Some("xmp"),
        "CreatorTool",
        "imageio-tests",
    )
    .expect("create tag");
    metadata
        .set_tag_with_path("xmp:CreatorTool", &tag)
        .expect("set metadata tag");
    metadata.into_metadata()
}
