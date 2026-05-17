mod common;

use imageio::prelude::*;

#[test]
fn heif_builder_parses_primary_and_canvas_values() {
    let properties = HeifBuilder::new()
        .expect("create HEIF builder")
        .primary_image(true)
        .loop_count(1)
        .canvas_size(128, 96)
        .build()
        .expect("build HEIF properties");
    let parsed = HeifProperties::from_properties(&properties).expect("parse HEIF properties");

    assert_eq!(parsed.is_primary, Some(true));
    assert_eq!(parsed.loop_count, Some(1));
    assert_eq!(parsed.canvas_pixel_width, Some(128));

    let destinations = imageio::heif::supported_destination_identifiers();
    assert!(destinations
        .iter()
        .all(|identifier| identifier.contains("hei")));
}
