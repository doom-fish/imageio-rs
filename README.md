# imageio

Safe Rust bindings for Apple's [ImageIO](https://developer.apple.com/documentation/imageio) framework on macOS.

```toml
[dependencies]
imageio = "0.12"
```

> **Status:** `imageio` follows the audited bridge pattern for the C-only `ImageIO.framework`.
>
> - the default build compiles a tiny SwiftPM bridge from `swift-bridge/`
> - ergonomic safe modules cover Source, Destination, Properties, Metadata, AuxiliaryData, ColorSync, AnimatedPNG, HEIF, `ProRAW`, and Thumbnail workflows
> - the optional `async` feature adds `async_api::IncrementalImageDecoder` for executor-agnostic progressive incremental decode updates
> - the optional `raw-ffi` feature preserves the audited C header surface in `imageio::ffi`
> - [`COVERAGE.md`](COVERAGE.md) tracks the audited rows from `CGImageSource.h`, `CGImageDestination.h`, `CGImageAnimation.h`, `CGImageMetadata.h`, and `CGImageProperties.h`

## Requirements

- macOS 13+
- Xcode command line tools / a working Swift toolchain
- Rust 1.82+

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
| Async incremental decode | `async_api` | Rust-only | `11_async_incremental_decoder` |

## High-level helpers

- `read_metadata(path)`
- `decode_bgra(path)`
- `decode_bgra_from_bytes(bytes)`
- `encode_bgra_to_bytes(bgra, width, height, format)`
- `convert_format(input, output, format)`
- `copy_image_source(input, output, format)`
- `ImageSource` + `SourceStatus` for file/data/incremental sources; `ImageSourceOptions` sets the type-identifier hint and `ShouldCache`, and `DataProvider` plus `ImageSource::from_data_provider` / `update_data_provider` cover `CGDataProvider` sources
- `ImageDestination` for file/data encodes, metadata, and auxiliary data; `ImageDestination::to_writer` streams the encoded bytes to any `Write + Send + 'static` value
- `ImageProperties` / `MutableProperties` plus typed APNG / HEIF / `ProRAW` / color helpers
- `Metadata`, `MetadataEnumerateOptions`, `MutableMetadata`, and `MetadataTag` for XMP workflows; mutable clones and `into_metadata()` produce independent trees
- `AuxiliaryDataInfo::set_color_space` / `color_space` retain the actual auxiliary `CGColorSpace`
- `create_thumbnail`, `animate_image`, and `animate_image_from_bytes`; animation callbacks follow native frame timing on the main queue
- `async_api::IncrementalImageDecoder` (feature = `async`) for progressive thumbnail snapshots from incremental sources

`CGImageDestinationCopyImageSource` is terminal: `ImageDestination::copy_image_source` writes the output and completes the destination without a later `finalize()`. Subsequent add/finalize calls return `ImageError::EncodeFailed`.

The animation helpers are synchronous and must be invoked on the process main thread. They return after finite native playback completes, when `ImageIO` ends playback early (for example a single-frame image), or after the callback returns `false`. `AnimationOptions::timeout` bounds the wait (60 seconds by default, `None` waits for playback to end); a timed-out call returns `ImageError::Timeout`, and no callback runs after the call returns.

## Decode limits

Every path that decodes a whole image (`ImageSource::decode_image_at_index`, `decode_bgra`, `decode_bgra_from_bytes`, `create_thumbnail`, `ImageDestination::add_image_from_source` and `convert_format`, and each frame of `animate_image`) is bounded by `DecodeLimits`. The defaults allow 16384 pixels per side and 256 MiB of BGRA output (8192x8192). The dimensions a file declares are checked before anything is decoded, so a small file that claims to be 40000x40000 pixels fails with `ImageError::LimitExceeded` instead of allocating gigabytes. Raise or lower the limits per source with `ImageSource::set_decode_limits`, or per animation with `AnimationOptions::limits`. Thumbnails are scaled down to fit the limits rather than refused. A decode still needs about twice the output size while the bridge's buffer is copied into the returned `Vec`.

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
- `11_async_incremental_decoder` (requires `--features async`)

Shared example output is written under `target/example-output`.

## Features

- default: safe Rust API backed by the Swift bridge in `swift-bridge/`
- `async`: enable `imageio::async_api::IncrementalImageDecoder` and its executor-agnostic progress stream
- `raw-ffi`: export the audited `ImageIO` C declarations in `imageio::ffi`

## Verification

The crate is verified with:

```bash
cargo build --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
for ex in {01_source_overview,02_destination_roundtrip,03_properties_view,04_metadata_roundtrip,05_auxiliary_data,06_color_sync,07_animated_png,08_heif,09_proraw,10_thumbnail}; do cargo run --example "$ex"; done
cargo run --example 11_async_incremental_decoder --features async
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
