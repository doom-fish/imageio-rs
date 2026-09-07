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

fn mutable_metadata(value: &str) -> MutableMetadata {
    let mut metadata = MutableMetadata::new().expect("create mutable metadata");
    metadata
        .register_namespace_for_prefix("http://ns.adobe.com/xap/1.0/", "xmp")
        .expect("register namespace");
    let tag = MetadataTag::new_string(
        "http://ns.adobe.com/xap/1.0/",
        Some("xmp"),
        "CreatorTool",
        value,
    )
    .expect("create metadata tag");
    metadata
        .set_tag_with_path("xmp:CreatorTool", &tag)
        .expect("set metadata tag");
    metadata
}

#[test]
fn mutable_metadata_clones_are_independent() {
    let original = mutable_metadata("original");
    let mut cloned = original.clone();
    cloned
        .set_string_value_with_path("xmp:CreatorTool", "clone")
        .expect("mutate clone");

    let original = original.into_metadata().expect("freeze original");
    let cloned = cloned.into_metadata().expect("freeze clone");

    assert_eq!(
        original
            .string_value_with_path("xmp:CreatorTool")
            .expect("read original"),
        Some("original".into())
    );
    assert_eq!(
        cloned
            .string_value_with_path("xmp:CreatorTool")
            .expect("read clone"),
        Some("clone".into())
    );
}

#[test]
fn frozen_metadata_is_independent_from_writable_clones() {
    let mutable = mutable_metadata("frozen");
    let mut writable = mutable.clone();
    let frozen = mutable.into_metadata().expect("freeze metadata");

    writable
        .set_string_value_with_path("xmp:CreatorTool", "mutated")
        .expect("mutate writable clone");

    assert_eq!(
        frozen
            .string_value_with_path("xmp:CreatorTool")
            .expect("read frozen metadata"),
        Some("frozen".into())
    );
}

#[test]
fn metadata_enumeration_tolerates_reentrant_mutation_of_a_writable_clone() {
    let mutable = mutable_metadata("enumerated");
    let mut writable = mutable.clone();
    let frozen = mutable.into_metadata().expect("freeze metadata");
    let mut callbacks = 0;

    frozen
        .enumerate_tags(None, |_, _| {
            callbacks += 1;
            writable
                .set_string_value_with_path("xmp:CreatorTool", "reentrant")
                .expect("mutate writable clone during enumeration");
            true
        })
        .expect("enumerate metadata");

    assert!(callbacks > 0);
    assert_eq!(
        frozen
            .string_value_with_path("xmp:CreatorTool")
            .expect("read enumerated metadata"),
        Some("enumerated".into())
    );
    assert_eq!(
        writable
            .into_metadata()
            .expect("freeze writable clone")
            .string_value_with_path("xmp:CreatorTool")
            .expect("read writable clone"),
        Some("reentrant".into())
    );
}
