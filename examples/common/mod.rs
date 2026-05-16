#![allow(dead_code)]

use imageio::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn output_dir(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("example-output")
        .join(name);
    fs::create_dir_all(&dir)?;
    Ok(dir)
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

pub fn sample_png_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    let image = sample_image();
    Ok(encode_bgra_to_bytes(
        &image.bgra,
        image.width,
        image.height,
        ImageFormat::Png,
    )?)
}

pub fn write_sample_png(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::write(path, sample_png_bytes()?)?;
    Ok(())
}

pub fn animated_gif_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("assets")
        .join("animated.gif")
}

pub fn sample_metadata() -> Result<Metadata, Box<dyn Error>> {
    let mut metadata = MutableMetadata::new()?;
    metadata.register_namespace_for_prefix("http://ns.adobe.com/xap/1.0/", "xmp")?;
    let tag = MetadataTag::new_string(
        "http://ns.adobe.com/xap/1.0/",
        Some("xmp"),
        "CreatorTool",
        "imageio-example",
    )?;
    metadata.set_tag_with_path("xmp:CreatorTool", &tag)?;
    Ok(metadata.into_metadata())
}
