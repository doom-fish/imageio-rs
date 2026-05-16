#![cfg(feature = "raw-ffi")]

//! Header-driven coverage harness for the optional raw `imageio::ffi` surface.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
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
    let path = sdk_root().join(format!(
        "System/Library/Frameworks/ImageIO.framework/Headers/{name}.h"
    ));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn read_rust_ffi() -> String {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ffi");
    let mut entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .collect::<Vec<_>>();
    entries.sort();
    entries
        .into_iter()
        .map(|path| {
            std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_header_functions(source: &str) -> BTreeSet<String> {
    let re = regex_lite::Regex::new(
        r"IMAGEIO_EXTERN[^;\n]*\b((?:CGAnimateImage[A-Za-z0-9_]+)|(?:CGImage(?:Source|Destination|Metadata)[A-Za-z0-9_]+))\s*\(",
    )
    .unwrap();
    re.captures_iter(source)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn extract_header_constants(source: &str) -> BTreeSet<String> {
    let re = regex_lite::Regex::new(r"IMAGEIO_EXTERN\s+const\s+CFStringRef\s+(k[A-Za-z0-9_]+)").unwrap();
    re.captures_iter(source)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn extract_header_enums(source: &str) -> BTreeMap<String, BTreeSet<String>> {
    let re = regex_lite::Regex::new(r"typedef\s+CF_ENUM\([^,]+,\s*([A-Za-z0-9_]+)\)\s*\{([^}]*)\}")
        .unwrap();
    re.captures_iter(source)
        .map(|capture| {
            let enum_name = capture[1].to_string();
            let cases = capture[2]
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with("//"))
                .map(|line| line.split("/*").next().unwrap().trim())
                .map(|line| line.trim_end_matches(','))
                .map(|line| line.split('=').next().unwrap().trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            (enum_name, cases)
        })
        .collect()
}

fn extract_rust_functions(source: &str) -> BTreeSet<String> {
    let re = regex_lite::Regex::new(
        r"pub\s+fn\s+((?:CGAnimateImage[A-Za-z0-9_]+)|(?:CGImage(?:Source|Destination|Metadata)[A-Za-z0-9_]+))\s*\(",
    )
    .unwrap();
    re.captures_iter(source)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn extract_rust_constants(source: &str) -> BTreeSet<String> {
    let re = regex_lite::Regex::new(r"pub\s+(?:static|const)\s+(k[A-Za-z0-9_]+)").unwrap();
    re.captures_iter(source)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn extract_rust_types(source: &str) -> BTreeSet<String> {
    let re = regex_lite::Regex::new(r"pub\s+type\s+([A-Za-z0-9_]+)\s*=").unwrap();
    re.captures_iter(source)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn assert_missing_empty(kind: &str, header: &str, missing: &BTreeSet<String>) {
    if !missing.is_empty() {
        eprintln!("missing {kind} for {header}:");
        for item in missing {
            eprintln!("  - {item}");
        }
    }
    assert!(missing.is_empty(), "{header}: missing {kind}");
}

fn assert_header_functions_covered(header: &str, rust_functions: &BTreeSet<String>) {
    let apple = extract_header_functions(&read_header(header));
    let missing = apple
        .difference(rust_functions)
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_missing_empty("functions", header, &missing);
}

fn assert_header_constants_covered(header: &str, rust_constants: &BTreeSet<String>) {
    let apple = extract_header_constants(&read_header(header));
    let missing = apple
        .difference(rust_constants)
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_missing_empty("constants", header, &missing);
}

fn assert_header_enums_covered(
    header: &str,
    rust_types: &BTreeSet<String>,
    rust_constants: &BTreeSet<String>,
) {
    let enums = extract_header_enums(&read_header(header));
    let missing_types = enums
        .keys()
        .filter(|name| !rust_types.contains(*name))
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_missing_empty("enum types", header, &missing_types);

    for (enum_name, cases) in enums {
        let missing_cases = cases
            .difference(rust_constants)
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_missing_empty(
            &format!("enum cases for {enum_name}"),
            header,
            &missing_cases,
        );
    }
}

#[test]
fn cg_image_source_header_coverage() {
    let rust = read_rust_ffi();
    let rust_functions = extract_rust_functions(&rust);
    let rust_constants = extract_rust_constants(&rust);
    let rust_types = extract_rust_types(&rust);

    assert_header_functions_covered("CGImageSource", &rust_functions);
    assert_header_constants_covered("CGImageSource", &rust_constants);
    assert_header_enums_covered("CGImageSource", &rust_types, &rust_constants);
}

#[test]
fn cg_image_destination_header_coverage() {
    let rust = read_rust_ffi();
    let rust_functions = extract_rust_functions(&rust);
    let rust_constants = extract_rust_constants(&rust);

    assert_header_functions_covered("CGImageDestination", &rust_functions);
    assert_header_constants_covered("CGImageDestination", &rust_constants);
}

#[test]
fn cg_image_animation_header_coverage() {
    let rust = read_rust_ffi();
    let rust_functions = extract_rust_functions(&rust);
    let rust_constants = extract_rust_constants(&rust);
    let rust_types = extract_rust_types(&rust);

    assert_header_functions_covered("CGImageAnimation", &rust_functions);
    assert_header_constants_covered("CGImageAnimation", &rust_constants);
    assert_header_enums_covered("CGImageAnimation", &rust_types, &rust_constants);
}

#[test]
fn cg_image_metadata_header_coverage() {
    let rust = read_rust_ffi();
    let rust_functions = extract_rust_functions(&rust);
    let rust_constants = extract_rust_constants(&rust);
    let rust_types = extract_rust_types(&rust);

    assert_header_functions_covered("CGImageMetadata", &rust_functions);
    assert_header_constants_covered("CGImageMetadata", &rust_constants);
    assert_header_enums_covered("CGImageMetadata", &rust_types, &rust_constants);
}

#[test]
fn cg_image_properties_header_coverage() {
    let rust = read_rust_ffi();
    let rust_constants = extract_rust_constants(&rust);

    assert_header_constants_covered("CGImageProperties", &rust_constants);
}
