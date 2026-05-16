#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = common::output_dir("10_thumbnail")?;
    let png = dir.join("sample.png");
    common::write_sample_png(&png)?;

    let source = ImageSource::from_path(&png)?;
    let thumbnail = create_thumbnail(&source, 0, ThumbnailOptions::new(1))?;
    let thumb_bytes = encode_bgra_to_bytes(
        &thumbnail.bgra,
        thumbnail.width,
        thumbnail.height,
        ImageFormat::Png,
    )?;
    fs::write(dir.join("thumbnail.png"), &thumb_bytes)?;
    println!("thumbnail={}x{}", thumbnail.width, thumbnail.height);
    Ok(())
}
