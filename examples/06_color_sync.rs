#[path = "common/mod.rs"]
mod common;

use imageio::color_sync;
use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut properties = MutableProperties::new()?;
    properties.set_string(color_sync::PROFILE_NAME_KEY, "Display P3")?;
    color_sync::set_decode_request(&mut properties, color_sync::DecodeRequest::Hdr)?;
    color_sync::set_encode_request(&mut properties, color_sync::EncodeRequest::IsoHdr)?;
    color_sync::set_optimize_color_for_sharing(&mut properties, true)?;

    let properties = properties.freeze()?;
    println!("profile={:?}", color_sync::profile_name(&properties));
    Ok(())
}
