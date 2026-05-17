mod common;

use imageio::color_sync;
use imageio::prelude::*;

#[test]
fn color_sync_helpers_manage_profile_keys() {
    let mut properties = MutableProperties::new().expect("create properties");
    properties
        .set_string(color_sync::PROFILE_NAME_KEY, "Display P3")
        .expect("set profile name");
    color_sync::set_decode_request(&mut properties, color_sync::DecodeRequest::Hdr)
        .expect("set decode request");
    color_sync::set_encode_request(&mut properties, color_sync::EncodeRequest::IsoHdr)
        .expect("set encode request");
    color_sync::set_optimize_color_for_sharing(&mut properties, true).expect("set optimize color");

    let properties = properties.freeze().expect("freeze color sync properties");
    assert_eq!(
        color_sync::profile_name(&properties),
        Some("Display P3".into())
    );
    assert_eq!(
        properties
            .string(color_sync::SOURCE_DECODE_REQUEST_KEY)
            .expect("decode request value"),
        Some(color_sync::SOURCE_DECODE_TO_HDR.into())
    );
}
