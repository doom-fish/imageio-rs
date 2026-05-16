# Changelog

## [0.4.0] - 2026-05-16

### Added

- A SwiftPM-backed `ImageIOBridge` build pipeline so the default crate path now
  follows the `screencapturekit-rs` bridge pattern for the C-only
  `ImageIO.framework`.
- Safe Rust modules for the requested logical areas:
  - Source
  - Destination
  - Properties
  - Metadata
  - AuxiliaryData
  - ColorSync
  - AnimatedPNG
  - HEIF
  - ProRAW
  - Thumbnail
- Ten numbered examples, one per logical area:
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
- Area-based smoke tests plus raw-header coverage validation behind the new
  `raw-ffi` feature.
- `COVERAGE.md`, documenting the audited `ImageIO` SDK rows from the active
  macOS SDK.

### Changed

- The full raw C surface is now explicitly gated behind the `raw-ffi` cargo
  feature while the default build goes through the Swift bridge.
- README now documents the bridge architecture, safe API split, numbered
  examples, and verification workflow.
- The header-driven coverage harness now checks `CGAnimateImage*` symbols in
  addition to the source, destination, metadata, and property families.

### Fixed

- Corrected ownership in mutable properties / metadata constructors so bridged
  handles are not released prematurely before later mutation or animation use.
- Cleaned up the new safe surface for `clippy --all-targets --all-features`.

## [0.3.0] - 2026-05-16

### Added

- Full audited raw `ImageIO.framework` header coverage for:
  - `CGImageSource.h`
  - `CGImageDestination.h`
  - `CGImageAnimation.h`
  - `CGImageMetadata.h`
  - `CGImageProperties.h`
- 748 exported `CFStringRef` constants in `imageio::ffi`, including the full
  `kCGImageProperty*` family.
- Raw declarations for all public `CGImageSource*`, `CGImageDestination*`,
  `CGAnimateImage*`, and `CGImageMetadata*` functions plus ImageIO enums.
- Safe `ImageSource` wrapper for file-backed, data-backed, and incremental
  sources with status inspection and frame decode.
- Safe `Metadata`, `MutableMetadata`, and `MetadataTag` wrappers for XMP
  round-tripping, namespace registration, path-based tag lookup, mutation, and
  block-based enumeration.
- Safe `animate_image` / `animate_image_from_bytes` helpers for
  `CGImageAnimation` that pump the main run loop until the callback stops
  playback.
- Safe `copy_image_source` helper for metadata-preserving source copies without
  forcing a full decode.
- New smoke examples:
  - `03_incremental_source`
  - `04_metadata`
  - `05_animation`
  - `06_copy_image_source`
- Generated animated GIF fixture at `examples/assets/animated.gif`.
- Header-driven API coverage tests for functions, constants, and enums across
  the full audited SDK surface.

### Changed

- README now documents the full audited surface, safe wrapper layers, and the
  expanded smoke example set.
- Existing examples now write into repo-local `target/example-output` instead of
  `/tmp`.
- Package metadata now includes `examples/**/*` and `tests/**/*` in published
  crates.

## [0.1.0] - Initial release

### Added

- `read_metadata(path)` -> `ImageMetadata { width, height, frame_count,
  has_alpha, source_format }`. Header-only read, no pixel decode.
- `decode_bgra(path)` -> `DecodedImage { width, height, bgra }`. Tightly
  packed premultiplied-alpha BGRA bytes. Drives ImageIO + a
  `CGBitmapContext` for guaranteed RGBA output regardless of source format.
- `convert_format(input, output, ImageFormat)` — round-trips through
  `CGImageSource` + `CGImageDestination`. Supports PNG, JPEG, HEIC,
  TIFF, GIF, BMP via the matching UTI strings.
- `ImageError` enum: `InvalidPath`, `OpenSourceFailed`, `NoImagesInSource`,
  `DecodeFailed`, `EncodeFailed`, `UnsupportedFormat`, `Unknown`.
- 2 examples (`01_read_image` + `02_convert_format`) using `sips` to
  synthesise a known-good source PNG.
- 2 API-coverage tests (`CGImageSource`, `CGImageDestination`) using the
  apple-cf / videotoolbox C-function-regex harness pattern.

### Why zero-Swift?

ImageIO is a pure C framework — `extern "C"` declarations link directly
against the system framework. No Swift bridge, no extra build.rs swift
build step, no Swift runtime rpath setup.
