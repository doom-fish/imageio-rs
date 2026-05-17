mod common;

use imageio::prelude::*;

#[test]
fn properties_support_nested_dictionaries() {
    let mut png = MutableProperties::new().expect("create png properties");
    png.set_i64("LoopCount", 4).expect("set loop count");

    let mut root = MutableProperties::new().expect("create root properties");
    root.set_dictionary("{PNG}", &png.freeze().expect("freeze nested properties"))
        .expect("set nested dictionary");
    root.set_string("ProfileName", "Display P3")
        .expect("set profile");
    let properties = root.freeze().expect("freeze root properties");

    assert_eq!(
        properties.string("ProfileName").expect("read profile"),
        Some("Display P3".into())
    );
    let nested = properties
        .dictionary("{PNG}")
        .expect("read nested dictionary")
        .expect("nested png");
    assert_eq!(nested.i64("LoopCount").expect("read loop count"), Some(4));
}
