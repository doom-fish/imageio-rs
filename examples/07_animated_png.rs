#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let props = AnimatedPngBuilder::new()?
        .loop_count(2)
        .delay_time(0.1)
        .canvas_size(64, 64)
        .build()?;
    let parsed =
        AnimatedPngProperties::from_properties(&props)?.ok_or("missing APNG properties")?;

    let mut frames = 0_usize;
    animate_image(common::animated_gif_path(), |_, _| {
        frames += 1;
        true
    })?;

    println!("loop_count={:?} frames={frames}", parsed.loop_count);
    Ok(())
}
