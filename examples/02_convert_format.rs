//! Convert an image file from one format to another.
//! Demonstrates round-trip PNG → JPEG → HEIC.
//!
//! Run: `cargo run --example 02_convert_format`

use imageio::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    std::fs::create_dir_all(&out_dir)?;
    let png = out_dir.join("imageio_convert.png");
    let _ = std::fs::remove_file(&png);

    println!("== Step 1: synthesise source PNG ==");
    let status = Command::new("sips")
        .args([
            "-s",
            "format",
            "png",
            "-Z",
            "512",
            "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns",
            "--out",
            png.to_str().unwrap(),
        ])
        .output()?;
    if !status.status.success() {
        eprintln!("sips stderr: {}", String::from_utf8_lossy(&status.stderr));
        return Err("sips failed".into());
    }
    println!(
        "source: {} ({} bytes)",
        png.display(),
        std::fs::metadata(&png)?.len()
    );

    for (name, fmt) in [
        ("imageio_convert.jpg", ImageFormat::Jpeg),
        ("imageio_convert.heic", ImageFormat::Heic),
        ("imageio_convert.tiff", ImageFormat::Tiff),
        ("imageio_convert.gif", ImageFormat::Gif),
        ("imageio_convert.bmp", ImageFormat::Bmp),
    ] {
        let out = out_dir.join(name);
        let _ = std::fs::remove_file(&out);
        match convert_format(&png, &out, fmt) {
            Ok(()) => {
                let size = std::fs::metadata(&out).map_or(0, |m| m.len());
                println!("  {fmt:>6?} -> {} ({size} bytes)", out.display());
            }
            Err(err) => println!("  {fmt:>6?} -> FAILED: {err}"),
        }
    }
    Ok(())
}
