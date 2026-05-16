#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = common::output_dir("02_destination_roundtrip")?;
    let image = common::sample_image();

    let mut destination = ImageDestination::to_data(ImageFormat::Jpeg.type_identifier(), 1)?;
    destination.add_image(&image, None)?;
    destination.finalize()?;
    let bytes = destination.data().ok_or("missing encoded bytes")?;
    fs::write(dir.join("sample.jpg"), &bytes)?;

    let source = ImageSource::from_bytes(&bytes)?;
    println!("encoded={} bytes decoded_type={:?}", bytes.len(), source.source_type());
    Ok(())
}
