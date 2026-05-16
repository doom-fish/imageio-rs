#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut info = AuxiliaryDataInfo::new()?;
    info.set_data(&[1, 2, 3, 4]);

    let mut description = MutableProperties::new()?;
    description.set_i64("Width", 2)?;
    description.set_i64("Height", 2)?;
    info.set_description(&description.freeze()?);
    info.set_metadata(&common::sample_metadata()?);

    println!(
        "aux_bytes={} has_metadata={} has_color_space={}",
        info.data().len(),
        info.metadata().is_some(),
        info.has_color_space()
    );
    Ok(())
}
