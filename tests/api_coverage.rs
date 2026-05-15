//! API-surface coverage harness for `imageio`.
//!
//! `ImageIO` is a pure C framework. Mirrors the apple-cf / videotoolbox
//! pattern: parse `IMAGEIO_EXTERN ... CGImageXxx(` C function signatures
//! from the headers and diff against our `extern "C"` declarations.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read_header(name: &str) -> String {
    let p = sdk_root().join(format!(
        "System/Library/Frameworks/ImageIO.framework/Headers/{name}.h"
    ));
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

fn extract_c_functions(prefix: &str, source: &str) -> BTreeSet<String> {
    let pattern = format!(r"\b({prefix}[A-Za-z0-9_]+)\s*\(");
    let re = regex_lite::Regex::new(&pattern).unwrap();
    re.captures_iter(source).map(|c| c[1].to_string()).collect()
}

fn extract_rust_externs() -> BTreeSet<String> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ffi/mod.rs");
    let s = std::fs::read_to_string(&p).unwrap();
    let re = regex_lite::Regex::new(r"pub\s+fn\s+([A-Za-z0-9_]+)\s*\(").unwrap();
    re.captures_iter(&s).map(|c| c[1].to_string()).collect()
}

fn report(
    name: &str,
    apple: &BTreeSet<String>,
    ours: &BTreeSet<String>,
    omitted: &BTreeSet<String>,
) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|s| !omitted.contains(*s))
        .collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, omitted={}, coverable={coverable}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        omitted.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for s in &missing {
            println!("  - {s}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

// ---- Tests ----

#[test]
fn cg_image_source_coverage() {
    let header = read_header("CGImageSource");
    let apple = extract_c_functions("CGImageSource", &header);
    let ours = extract_rust_externs();
    let omitted = omitted_set([
        // Type identifier — every CG type has it; never used directly.
        "CGImageSourceGetTypeID",
        // CGDataProvider-based source — v0.2 (currently URL + Data only).
        "CGImageSourceCreateWithDataProvider",
        // Async-progressive sources for incremental network decode — v0.2.
        "CGImageSourceCreateIncremental",
        "CGImageSourceUpdateData",
        "CGImageSourceUpdateDataProvider",
        "CGImageSourceGetStatus",
        "CGImageSourceGetStatusAtIndex",
        // Auxiliary data, depth/disparity, primary-image accessors — v0.2.
        "CGImageSourceCopyAuxiliaryDataInfoAtIndex",
        "CGImageSourceCreateThumbnailAtIndex",
        "CGImageSourceGetPrimaryImageIndex",
        "CGImageSourceCopyMetadataAtIndex",
        "CGImageSourceRemoveCacheAtIndex",
        // Internal / removed:
        "CGImageSourceSetAllowableTypes",
    ]);
    report("CGImageSource", &apple, &ours, &omitted);
}

#[test]
fn cg_image_destination_coverage() {
    let header = read_header("CGImageDestination");
    let apple = extract_c_functions("CGImageDestination", &header);
    let ours = extract_rust_externs();
    let omitted = omitted_set([
        "CGImageDestinationGetTypeID",
        // CGDataConsumer-based destination — v0.2 (currently URL + Data only).
        "CGImageDestinationCreateWithDataConsumer",
        // Animation, auxiliary data, metadata variants — v0.2.
        "CGImageDestinationAddImageFromSource",
        "CGImageDestinationAddImageAndMetadata",
        "CGImageDestinationAddAuxiliaryDataInfo",
        "CGImageDestinationAddImageAndAuxiliaryData",
        "CGImageDestinationFinalizeAsynchronously",
        "CGImageDestinationCopyImageSource",
        // Animation-frame APIs land with multi-frame support.
        "CGImageDestinationFinalizeAsynchronouslyWithCompletionHandler",
        // Per-image properties (compression quality, dest profile) — v0.2.
        "CGImageDestinationSetProperties",
    ]);
    report("CGImageDestination", &apple, &ours, &omitted);
}
