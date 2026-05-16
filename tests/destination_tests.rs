mod common;

use imageio::prelude::*;

#[test]
fn destination_round_trips_encoded_data() {
    let image = common::sample_image();
    let mut destination = ImageDestination::to_data(ImageFormat::Jpeg.type_identifier(), 1)
        .expect("create jpeg destination");
    destination.add_image(&image, None).expect("add image");
    destination.finalize().expect("finalize destination");
    let bytes = destination.data().expect("destination data");

    let source = ImageSource::from_bytes(&bytes).expect("open encoded bytes");
    assert!(source.frame_count() >= 1);
    assert!(source.source_type().is_some());
}
