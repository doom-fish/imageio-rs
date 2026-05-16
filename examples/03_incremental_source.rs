//! Feed an incremental `CGImageSource` in two chunks.

use imageio::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    std::fs::create_dir_all(&out_dir)?;
    let png = out_dir.join("imageio_incremental.png");
    let _ = std::fs::remove_file(&png);

    let status = Command::new("sips")
        .args([
            "-s",
            "format",
            "png",
            "-Z",
            "128",
            "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns",
            "--out",
            png.to_str().unwrap(),
        ])
        .output()?;
    if !status.status.success() {
        eprintln!("sips stderr: {}", String::from_utf8_lossy(&status.stderr));
        return Err("sips failed".into());
    }

    let bytes = std::fs::read(&png)?;
    let midpoint = bytes.len() / 2;
    let mut source = ImageSource::incremental()?;
    source.update_data(&bytes[..midpoint], false)?;
    println!("status after first chunk: {:?}", source.status());

    source.update_data(&bytes, true)?;
    println!("status after final chunk: {:?}", source.status());
    println!("source type: {:?}", source.source_type());
    println!("frame count: {}", source.frame_count());

    let decoded = source.decode_image_at_index(0)?;
    println!(
        "decoded incremental image: {}x{} ({} bytes)",
        decoded.width,
        decoded.height,
        decoded.bgra.len()
    );
    assert_eq!(source.status(), SourceStatus::Complete);
    Ok(())
}
