#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = HeifProperties::from_properties(
        &HeifBuilder::new()?
            .primary_image(true)
            .loop_count(1)
            .canvas_size(128, 96)
            .build()?,
    )?;

    let heif_destinations = imageio::heif::supported_destination_identifiers();
    if heif_destinations.iter().any(|identifier| identifier == "public.heic") {
        let image = common::sample_image();
        let bytes = encode_bgra_to_bytes(&image.bgra, image.width, image.height, ImageFormat::Heic)?;
        println!("heic_bytes={} parsed_primary={:?}", bytes.len(), parsed.is_primary);
    } else {
        println!("heic unsupported parsed_primary={:?}", parsed.is_primary);
    }
    Ok(())
}
