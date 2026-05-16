#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = common::output_dir("01_source_overview")?;
    let png = dir.join("sample.png");
    common::write_sample_png(&png)?;

    let source = ImageSource::from_path(&png)?;
    let decoded = source.decode_image_at_index(0)?;

    println!(
        "type={:?} frames={} status={:?} decoded={}x{}",
        source.source_type(),
        source.frame_count(),
        source.status(),
        decoded.width,
        decoded.height
    );
    Ok(())
}
