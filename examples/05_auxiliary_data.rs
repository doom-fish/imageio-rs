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
    info.set_color_space(&CGColorSpace::device_gray());

    println!(
        "aux_bytes={} has_metadata={} color_components={:?}",
        info.data().len(),
        info.metadata().is_some(),
        info.color_space()
            .map(|color_space| color_space.number_of_components())
    );
    Ok(())
}
