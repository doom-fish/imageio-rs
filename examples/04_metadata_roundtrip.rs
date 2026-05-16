#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = common::sample_metadata()?;
    let xmp = metadata.create_xmp_data()?;
    let roundtrip = Metadata::from_xmp_data(&xmp)?;

    println!(
        "xmp_bytes={} creator_tool={:?}",
        xmp.len(),
        roundtrip.string_value_with_path("xmp:CreatorTool")?
    );
    Ok(())
}
