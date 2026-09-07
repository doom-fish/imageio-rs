mod common;

use imageio::prelude::*;

#[test]
fn auxiliary_data_round_trips_data_and_description() {
    let mut info = AuxiliaryDataInfo::new().expect("create auxiliary data info");
    info.set_data(&[1, 2, 3, 4]);
    let mut description = MutableProperties::new().expect("create auxiliary description");
    description.set_i64("Width", 2).expect("set width");
    info.set_description(&description.freeze().expect("freeze auxiliary description"));
    info.set_metadata(&common::sample_metadata());

    assert_eq!(info.data(), vec![1, 2, 3, 4]);
    assert_eq!(
        info.description()
            .expect("description")
            .i64("Width")
            .expect("width in description"),
        Some(2)
    );
    assert!(info.metadata().is_some());
}

#[test]
fn auxiliary_data_retains_the_actual_non_rgb_color_space() {
    let mut info = AuxiliaryDataInfo::new().expect("create auxiliary data info");
    {
        let gray = CGColorSpace::device_gray();
        assert_eq!(gray.number_of_components(), 1);
        info.set_color_space(&gray);
    }

    let retained = info.color_space().expect("retained color space");
    assert!(info.has_color_space());
    assert_eq!(retained.number_of_components(), 1);
}
