# imageio

Safe Rust bindings for Apple's [ImageIO](https://developer.apple.com/documentation/imageio) framework on macOS — read, write, and convert images in any format the OS supports (PNG, JPEG, HEIC, TIFF, GIF, BMP, RAW, …).

> **Status:** experimental. v0.1 ships file-based read/write/convert plus tight BGRA decode. Animation frames, EXIF/IPTC metadata, write-side properties (compression quality, EXIF preservation) land in v0.2.

ImageIO is pure C, so this crate is **zero-Swift** — just thin `extern "C"` declarations linked against the system frameworks.

## Quick start

```rust,no_run
use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Read metadata without decoding pixels (cheap).
    let meta = read_metadata("/tmp/photo.heic")?;
    println!("{}x{}, alpha={}, format={:?}",
        meta.width, meta.height, meta.has_alpha, meta.source_format);

    // 2. Decode to tightly packed BGRA bytes (premultiplied alpha).
    let img = decode_bgra("/tmp/photo.heic")?;
    println!("decoded {} bytes ({}x{})", img.bgra.len(), img.width, img.height);

    // 3. Convert format (HEIC → PNG).
    convert_format("/tmp/photo.heic", "/tmp/photo.png", ImageFormat::Png)?;
    Ok(())
}
```

## Supported formats

Whatever `ImageIO` supports on the running macOS version — typically:
PNG, JPEG, HEIC/HEIF, AVIF (macOS 13+), TIFF, GIF, BMP, ICNS, RAW (CR2, NEF, ARW, …), PSD (read-only), PDF (single-page rasterise).

## Pipeline composition

```text
imageio (load file) ──► DecodedImage(BGRA bytes)
                              │
                              ├─► apple-vision (OCR / face detection / barcodes)
                              ├─► coreimage-rs (filters)
                              └─► your own pipeline
```

Pairs naturally with [`apple-vision`](https://github.com/doom-fish/vision-rs): replaces the ad-hoc Swift `CIImage(contentsOf:)` shim that crate ships with, so you can decode → preprocess → run Vision requests entirely in safe Rust.

## Roadmap

- [x] `read_metadata(path)` — width, height, frame count, alpha, format UTI
- [x] `decode_bgra(path)` — pure pixel data
- [x] `convert_format(in, out, format)` — file-to-file conversion
- [ ] EXIF / IPTC / GPS metadata extraction
- [ ] Multi-frame iteration (animated GIF, multi-page TIFF)
- [ ] Write-side properties (compression quality, color profile preservation)
- [ ] In-memory decode (`CFDataRef` sources)
- [ ] Streaming `CGImageSource` for incremental network decode

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
