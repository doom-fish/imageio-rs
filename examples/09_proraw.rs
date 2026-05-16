#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let properties = ProRawBuilder::new()?
        .profile_name("Synthetic Camera Profile")
        .unique_camera_model("Synthetic Camera")
        .build()?;
    let parsed = ProRawProperties::from_properties(&properties)?;
    println!(
        "profile={:?} model={:?} supported={:?}",
        parsed.profile_name,
        parsed.unique_camera_model,
        imageio::proraw::supported_source_identifiers()
    );
    Ok(())
}
