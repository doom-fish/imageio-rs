mod common;

use imageio::prelude::*;

#[test]
fn proraw_builder_parses_profile_and_model() {
    let properties = ProRawBuilder::new()
        .expect("create ProRAW builder")
        .profile_name("Synthetic Camera Profile")
        .unique_camera_model("Synthetic Camera")
        .build()
        .expect("build ProRAW properties");
    let parsed = ProRawProperties::from_properties(&properties).expect("parse ProRAW properties");

    assert!(parsed.has_raw_dictionary);
    assert!(parsed.has_dng_dictionary);
    assert_eq!(parsed.profile_name, Some("Synthetic Camera Profile".into()));
    assert_eq!(parsed.unique_camera_model, Some("Synthetic Camera".into()));
}
