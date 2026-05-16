//! Copy an encoded image source without forcing a full decode.

use imageio::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/example-output");
    std::fs::create_dir_all(&out_dir)?;
    let src = out_dir.join("imageio_copy_source.png");
    let dst = out_dir.join("imageio_copy_source_copy.png");
    let _ = std::fs::remove_file(&src);
    let _ = std::fs::remove_file(&dst);

    let status = Command::new("sips")
        .args([
            "-s",
            "format",
            "png",
            "-Z",
            "192",
            "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/ToolbarCustomizeIcon.icns",
            "--out",
            src.to_str().unwrap(),
        ])
        .output()?;
    if !status.status.success() {
        eprintln!("sips stderr: {}", String::from_utf8_lossy(&status.stderr));
        return Err("sips failed".into());
    }

    copy_image_source(&src, &dst, None)?;
    let original = read_metadata(&src)?;
    let copied = read_metadata(&dst)?;
    println!("original: {original:?}");
    println!("copied:   {copied:?}");
    assert_eq!(original.width, copied.width);
    assert_eq!(original.height, copied.height);
    assert_eq!(original.frame_count, copied.frame_count);
    assert_eq!(original.source_format, copied.source_format);
    Ok(())
}
