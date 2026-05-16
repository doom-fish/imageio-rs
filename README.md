# imageio

Safe Rust bindings for Apple's [ImageIO](https://developer.apple.com/documentation/imageio) framework on macOS.

> **Status:** `imageio` `0.3.0` covers the current public `ImageIO.framework` headers audited from the active macOS SDK:
>
> - `CGImageSource.h`
> - `CGImageDestination.h`
> - `CGImageAnimation.h`
> - `CGImageMetadata.h`
> - `CGImageProperties.h`

`ImageIO` is a pure C framework, so this crate stays **zero-Swift**. The full SDK surface is available through `imageio::ffi`, while the crate also layers safe Rust helpers for the most common workflows.

## High-level API

- `read_metadata(path)`
- `decode_bgra(path)`
- `decode_bgra_from_bytes(bytes)`
- `encode_bgra_to_bytes(bgra, width, height, format)`
- `convert_format(input, output, format)`
- `copy_image_source(input, output, format)`
- `ImageSource` + `SourceStatus` for file/data/incremental sources
- `Metadata`, `MutableMetadata`, `MetadataTag` for `CGImageMetadata`
- `animate_image` / `animate_image_from_bytes` for `CGImageAnimation`

## Quick start

```rust,no_run
use imageio::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    let input = output.join("photo.heic");
    let png = output.join("photo.png");

    let meta = read_metadata(&input)?;
    println!("{}x{}, alpha={}, format={:?}", meta.width, meta.height, meta.has_alpha, meta.source_format);

    let decoded = decode_bgra(&input)?;
    println!("decoded {} bytes", decoded.bgra.len());

    convert_format(&input, &png, ImageFormat::Png)?;
    Ok(())
}
```

## Lower-level examples

The shipped smoke examples exercise the safe wrappers and the raw framework paths end-to-end:

- `01_read_image`
- `02_convert_format`
- `02_data_round_trip`
- `03_incremental_source`
- `04_metadata`
- `05_animation`
- `06_copy_image_source`

All examples write outputs under `target/example-output`.

## Raw FFI

For APIs that are not wrapped ergonomically yet, use `imageio::ffi`. The crate exposes the full audited function / constant / enum surface from the headers above, including:

- `CGImageSource*`
- `CGImageDestination*`
- `CGAnimateImage*`
- `CGImageMetadata*`
- `kCGImageProperty*` / `kCGImageSource*` / `kCGImageDestination*` constants

## Verification

The crate is verified with:

```bash
cargo build --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run --example 01_read_image
cargo run --example 02_convert_format
cargo run --example 02_data_round_trip
cargo run --example 03_incremental_source
cargo run --example 04_metadata
cargo run --example 05_animation
cargo run --example 06_copy_image_source
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
