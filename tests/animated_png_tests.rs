mod common;

use imageio::prelude::*;

#[test]
fn animated_png_builder_and_animation_helper_work() {
    let properties = AnimatedPngBuilder::new()
        .expect("create APNG builder")
        .loop_count(2)
        .delay_time(0.1)
        .canvas_size(64, 64)
        .build()
        .expect("build APNG properties");
    let parsed = AnimatedPngProperties::from_properties(&properties)
        .expect("parse APNG properties")
        .expect("APNG dictionary present");

    let mut frames = 0_usize;
    animate_image(common::animated_gif_path(), |_, _| {
        frames += 1;
        true
    })
    .expect("animate GIF");

    assert_eq!(parsed.loop_count, Some(2));
    assert!(frames > 0);
}
