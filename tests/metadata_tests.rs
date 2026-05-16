mod common;

use imageio::prelude::*;

#[test]
fn metadata_round_trips_xmp_data() {
    let metadata = common::sample_metadata();
    let xmp = metadata.create_xmp_data().expect("create xmp data");
    let roundtrip = Metadata::from_xmp_data(&xmp).expect("roundtrip metadata");

    assert_eq!(
        roundtrip
            .string_value_with_path("xmp:CreatorTool")
            .expect("read creator tool"),
        Some("imageio-tests".into())
    );
    assert!(!roundtrip.tags().is_empty());
}
