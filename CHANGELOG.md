# Changelog

## [Unreleased]

### Fixed

- Fixed BGRA buffers being read and written as RGBA. `decodeCGImageToBGRA`, `makeCGImage(fromBGRA:)`, and `cg_image_to_bgra` all declared `premultipliedLast | byteOrder32Big`, which is RGBA in memory, so every `DecodedImage.bgra` had its red and blue channels swapped. They now declare `premultipliedFirst | byteOrder32Little`, the packing Core Video calls `32BGRA`. Encode and decode were mutually consistent, so round-trips through this crate hid the swap; it only showed up against real BGRA sources such as `ScreenCaptureKit`.

## [0.9.1] - 2026-05-20

- Added in-`src/` unit tests across `color_sync`, `error`, `image`, `metadata`, `source`, and `thumbnail` (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.9.0] - 2026-05-20

### Added

- Added feature-gated `async_api::IncrementalImageDecoder`, an executor-agnostic incremental decode helper that snapshots `CGImageSource` status and progressive thumbnail availability after each `CGImageSourceUpdateData` call.
- Added `examples/11_async_incremental_decoder.rs` and `tests/async_api_tests.rs` covering the new incremental async surface with `pollster::block_on`.

### Changed

- Documented the new `async` cargo feature and its verification flow in the README.

## [0.8.3] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.8.2] - 2026-05-18

### Changed

- Added one-line rustdoc coverage across the public safe surface, documenting the exported ImageIO wrappers and helpers.

## [0.8.1] - 2026-05-18

### Changed

- chore: re-export OS primitives (OSStatus) from apple-cf

## [0.8.0] - 2026-05-18

### Changed

- Re-exported `CFRange`, `CGContextRef`, and `CGColorSpaceRef` from `apple_cf::raw`, removing the remaining crate-local duplicate CoreFoundation/CoreGraphics typedefs.
- Raised the `apple-cf` dependency to `>=0.9, <0.10` so raw ImageIO interop uses the shared `CGContextRef` definition from `apple-cf` 0.9.

## [0.7.0] - 2026-05-18

### Changed

- Re-exported CoreFoundation `CF*Ref` aliases in `imageio::ffi` from `apple_cf::raw`, removing the crate-local duplicate typedefs and aligning raw interop with `apple-cf`'s opaque-pointer types.

## [0.6.0] - 2026-05-18

### Changed

- Re-bump for `apple-cf` 0.8.0 nested `CGRect` — the re-exported `CGRect` type now has `{ origin, size }` nested fields instead of flat `{ x, y, width, height }`.

## [0.5.0] - 2026-05-18

### Changed

- Re-exported `CGPoint`, `CGRect`, and `CGSize` from `apple_cf::cg` in `src/ffi/mod.rs`, completing the CoreGraphics geometry migration and removing crate-local duplicate definitions.
- Updated internal CoreGraphics call sites to construct geometry via the shared `apple_cf::cg` APIs.

## [0.4.2] - 2025-05-17

### Fixed

- Added panic-safe wrappers to FFI callbacks (`animation_trampoline` and `metadata::enumerate::trampoline`) using `doom-fish-utils::panic_safe::catch_user_panic` to prevent unhandled panics from unwinding across the C ABI boundary.
- Fixed clippy `cast_possible_wrap` warning in destination tests by using `isize::try_from` instead of `as isize` cast.

## [0.4.1] - 2026-05-16

### Added

- Safe metadata coverage for `kCFErrorDomainCGImageMetadata` via `Metadata::error_domain()`.
- Recursive metadata tag traversal via `MetadataEnumerateOptions` and `Metadata::enumerate_tags_with_options()`.
- Raw `imageio::ffi` exports for `kCFErrorDomainCGImageMetadata` and `kCGImageMetadataEnumerateRecursively`.

### Changed

- The metadata example and metadata tests now exercise recursive tag enumeration.
- The raw-header coverage harness now matches nullability-annotated `CFStringRef` constants in `CGImageMetadata.h`.

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
