//! Convert an image file from one format to another.
//! Demonstrates round-trip PNG → JPEG → HEIC.
//!
//! Run: `cargo run --example 02_convert_format`

use std::path::PathBuf;
use std::process::Command;
use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png: PathBuf = "/tmp/imageio_convert.png".into();
    let _ = std::fs::remove_file(&png);

    println!("== Step 1: synthesise source PNG ==");
    Command::new("sips")
        .args([
            "-s", "format", "png", "-Z", "512",
            "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns",
            "--out", png.to_str().unwrap(),
        ])
        .status()?;
    println!("source: {} ({} bytes)", png.display(), std::fs::metadata(&png)?.len());

    for (out, fmt) in [
        ("/tmp/imageio_convert.jpg", ImageFormat::Jpeg),
        ("/tmp/imageio_convert.heic", ImageFormat::Heic),
        ("/tmp/imageio_convert.tiff", ImageFormat::Tiff),
        ("/tmp/imageio_convert.gif", ImageFormat::Gif),
        ("/tmp/imageio_convert.bmp", ImageFormat::Bmp),
    ] {
        let _ = std::fs::remove_file(out);
        match convert_format(&png, out, fmt) {
            Ok(()) => {
                let size = std::fs::metadata(out).map_or(0, |m| m.len());
                println!("  {fmt:>6?} -> {out} ({size} bytes)");
            }
            Err(e) => println!("  {fmt:>6?} -> FAILED: {e}"),
        }
    }
    Ok(())
}
