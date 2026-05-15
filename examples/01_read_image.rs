//! Smoke test: render a known-good test image via Apple's `sips` CLI,
//! then read its metadata + decode its pixels.
//!
//! Run: `cargo run --example 01_read_image`

use std::path::PathBuf;
use std::process::Command;
use imageio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png: PathBuf = "/tmp/imageio_smoke.png".into();
    let _ = std::fs::remove_file(&png);

    println!("== Step 1: synthesise a 256x256 test PNG via sips ==");
    // sips can convert from any system image. Use the standard system
    // icon as a source.
    let src = "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns";
    let status = Command::new("sips")
        .args(["-s", "format", "png", "-Z", "256", src, "--out", png.to_str().unwrap()])
        .output()?;
    if !status.status.success() {
        eprintln!("sips stderr: {}", String::from_utf8_lossy(&status.stderr));
        return Err("sips failed".into());
    }
    println!("synthesised {} ({} bytes)",
        png.display(), std::fs::metadata(&png)?.len());

    println!("\n== Step 2: read metadata ==");
    let meta = read_metadata(&png)?;
    println!("  width:     {}", meta.width);
    println!("  height:    {}", meta.height);
    println!("  frames:    {}", meta.frame_count);
    println!("  has alpha: {}", meta.has_alpha);
    println!("  format:    {:?}", meta.source_format);

    println!("\n== Step 3: decode pixels ==");
    let img = decode_bgra(&png)?;
    println!("  decoded {} bytes ({}x{}, {} bytes/row)",
        img.bgra.len(), img.width, img.height, img.bytes_per_row());
    assert_eq!(img.bgra.len(), img.width * img.height * 4);
    println!("  OK pixel buffer length matches dimensions");

    Ok(())
}
