mod common;

use imageio::prelude::*;

#[test]
fn animated_png_builder_round_trips_properties() {
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

    assert_eq!(parsed.loop_count, Some(2));
}

#[test]
fn synchronous_animation_rejects_non_main_thread_callers() {
    let mut called = false;
    let error = animate_image(common::animated_gif_path(), |_, _| {
        called = true;
        false
    })
    .expect_err("worker-thread animation must fail");

    assert!(!called);
    assert!(matches!(
        error,
        ImageError::DecodeFailed(message) if message.contains("process main thread")
    ));
}
