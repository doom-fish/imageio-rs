#[path = "common/mod.rs"]
mod common;

use imageio::prelude::*;

const fn nested_xmp() -> &'static [u8] {
    br#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
<rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/">
<dc:title>
<rdf:Alt>
<rdf:li xml:lang="x-default">Hello</rdf:li>
<rdf:li xml:lang="en-US">Hello EN</rdf:li>
</rdf:Alt>
</dc:title>
</rdf:Description>
</rdf:RDF>
</x:xmpmeta>"#
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = common::sample_metadata()?;
    let xmp = metadata.create_xmp_data()?;
    let roundtrip = Metadata::from_xmp_data(&xmp)?;

    let nested = Metadata::from_xmp_data(nested_xmp())?;
    let mut recursive_paths = Vec::new();
    nested.enumerate_tags_with_options(
        None,
        MetadataEnumerateOptions::recursive(),
        |path, _tag| {
            recursive_paths.push(path);
            true
        },
    )?;

    println!(
        "xmp_bytes={} creator_tool={:?} error_domain={} recursive_paths={:?}",
        xmp.len(),
        roundtrip.string_value_with_path("xmp:CreatorTool")?,
        Metadata::error_domain(),
        recursive_paths
    );
    Ok(())
}
