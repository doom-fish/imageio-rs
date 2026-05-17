#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut png = MutableProperties::new()?;
    png.set_i64("LoopCount", 3)?;
    png.set_f64("DelayTime", 0.25)?;

    let mut root = MutableProperties::new()?;
    root.set_dictionary("{PNG}", &png.freeze()?)?;
    root.set_string("ProfileName", "Display P3")?;
    let properties = root.freeze()?;

    println!(
        "keys={:?} profile={:?} loop_count={:?}",
        properties.keys(),
        properties.string("ProfileName")?,
        properties
            .dictionary("{PNG}")?
            .and_then(|png| png.i64("LoopCount").ok().flatten())
    );
    Ok(())
}
