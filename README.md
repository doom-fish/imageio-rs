# imageio

Safe Rust bindings for Apple's [ImageIO](https://developer.apple.com/documentation/imageio) framework on macOS.

> **Status:** `imageio` `0.4.0` follows the `screencapturekit-rs` coverage pattern for the C-only `ImageIO.framework`.
>
> - the default build compiles a tiny SwiftPM bridge from `swift-bridge/`
> - ergonomic safe modules cover Source, Destination, Properties, Metadata, AuxiliaryData, ColorSync, AnimatedPNG, HEIF, `ProRAW`, and Thumbnail workflows
> - the optional `raw-ffi` feature preserves the audited C header surface in `imageio::ffi`
> - [`COVERAGE.md`](COVERAGE.md) tracks the audited rows from `CGImageSource.h`, `CGImageDestination.h`, `CGImageAnimation.h`, `CGImageMetadata.h`, and `CGImageProperties.h`

## Requirements

- macOS 13+
- Xcode command line tools / a working Swift toolchain

## Safe API areas

| Area | Rust module(s) | Swift bridge file | Example |
| --- | --- | --- | --- |
| Source | `source`, `image` | `Source.swift` | `01_source_overview` |
| Destination | `destination`, `image` | `Destination.swift` | `02_destination_roundtrip` |
| Properties | `properties` | `Properties.swift` | `03_properties_view` |
| Metadata | `metadata` | `Metadata.swift` | `04_metadata_roundtrip` |
| AuxiliaryData | `auxiliary_data` | `AuxiliaryData.swift` | `05_auxiliary_data` |
| ColorSync | `color_sync` | `ColorSync.swift` | `06_color_sync` |
| AnimatedPNG | `animated_png`, `animation` | `AnimatedPNG.swift` | `07_animated_png` |
| HEIF | `heif` | `HEIF.swift` | `08_heif` |
| `ProRAW` | `proraw` | `ProRAW.swift` | `09_proraw` |
| Thumbnail | `thumbnail` | `Thumbnail.swift` | `10_thumbnail` |

## High-level helpers

- `read_metadata(path)`
- `decode_bgra(path)`
- `decode_bgra_from_bytes(bytes)`
- `encode_bgra_to_bytes(bgra, width, height, format)`
- `convert_format(input, output, format)`
- `copy_image_source(input, output, format)`
- `ImageSource` + `SourceStatus` for file/data/incremental sources
- `ImageDestination` for file/data encodes, metadata, and auxiliary data
- `ImageProperties` / `MutableProperties` plus typed APNG / HEIF / `ProRAW` / color helpers
- `Metadata`, `MutableMetadata`, and `MetadataTag` for XMP workflows
- `create_thumbnail`, `animate_image`, and `animate_image_from_bytes`

## Quick start

```rust,no_run
use imageio::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    let input = output.join("photo.heic");
    let png = output.join("photo.png");

    let meta = read_metadata(&input)?;
    println!(
        "{}x{}, alpha={}, format={:?}",
        meta.width, meta.height, meta.has_alpha, meta.source_format
    );

    let decoded = decode_bgra(&input)?;
    println!("decoded {} bytes", decoded.bgra.len());

    convert_format(&input, &png, ImageFormat::Png)?;
    Ok(())
}
```

## Examples

The numbered smoke examples cover every logical area and all exit successfully on headless macOS:

- `01_source_overview`
- `02_destination_roundtrip`
- `03_properties_view`
- `04_metadata_roundtrip`
- `05_auxiliary_data`
- `06_color_sync`
- `07_animated_png`
- `08_heif`
- `09_proraw`
- `10_thumbnail`

Shared example output is written under `target/example-output`.

## Features

- default: safe Rust API backed by the Swift bridge in `swift-bridge/`
- `raw-ffi`: export the audited `ImageIO` C declarations in `imageio::ffi`

## Verification

The crate is verified with:

```bash
cargo clippy --all-targets -- -D warnings
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo test --features raw-ffi
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
