//! Create, serialize, parse, and enumerate `ImageIO` metadata.

use imageio::ffi;
use imageio::{Metadata, MetadataTag, MutableMetadata};
use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata = MutableMetadata::new()?;
    metadata.register_namespace_for_prefix("http://ns.adobe.com/tiff/1.0/", "tiff")?;

    let artist = MetadataTag::new_string(
        "http://ns.adobe.com/tiff/1.0/",
        Some("tiff"),
        "Artist",
        "imageio-rs smoke",
    )?;
    metadata.set_tag_with_path("tiff:Artist", &artist)?;
    unsafe {
        metadata.set_string_value_matching_image_property(
            ffi::kCGImagePropertyTIFFDictionary,
            ffi::kCGImagePropertyTIFFSoftware,
            "imageio-rs smoke",
        )?;
    }

    let metadata = metadata.into_metadata();
    let xmp = metadata.create_xmp_data()?;
    println!("serialized XMP bytes: {}", xmp.len());

    let parsed = Metadata::from_xmp_data(&xmp)?;
    println!(
        "creator tool at path: {:?}",
        parsed.string_value_with_path("xmp:CreatorTool")?
    );

    let seen = Rc::new(RefCell::new(Vec::new()));
    let seen_for_block = Rc::clone(&seen);
    parsed.enumerate_tags(None, move |path, tag| {
        println!("  {path} => {:?}", tag.string_value());
        seen_for_block.borrow_mut().push(path);
        true
    })?;

    assert!(seen.borrow().iter().any(|path| path == "dc:creator"));
    assert!(seen.borrow().iter().any(|path| path == "xmp:CreatorTool"));
    Ok(())
}
