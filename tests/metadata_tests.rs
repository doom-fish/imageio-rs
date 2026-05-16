mod common;

use std::collections::BTreeSet;

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

#[test]
fn metadata_exposes_error_domain_and_recursive_enumeration() {
    assert_eq!(Metadata::error_domain(), "kCFErrorDomainCGImageMetadata");

    let metadata = Metadata::from_xmp_data(nested_xmp()).expect("parse nested xmp");

    let mut top_level = BTreeSet::new();
    metadata
        .enumerate_tags(None, |path, _tag| {
            top_level.insert(path);
            true
        })
        .expect("enumerate top-level tags");
    assert_eq!(
        top_level,
        BTreeSet::from([String::from("dc:title"), String::from("iio:hasXMP"),])
    );

    let mut recursive = BTreeSet::new();
    metadata
        .enumerate_tags_with_options(None, MetadataEnumerateOptions::recursive(), |path, _tag| {
            recursive.insert(path);
            true
        })
        .expect("enumerate recursive tags");
    assert_eq!(
        recursive,
        BTreeSet::from([
            String::from("dc:title"),
            String::from("dc:title[en-US]"),
            String::from("dc:title[x-default]"),
            String::from("iio:hasXMP"),
        ])
    );
}
