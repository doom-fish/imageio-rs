# Changelog

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
