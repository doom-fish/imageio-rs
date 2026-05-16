use imageio::{decode_bgra_from_bytes, encode_bgra_to_bytes, ImageFormat};

fn main() -> Result<(), imageio::ImageError> {
    let w = 64usize;
    let h = 64usize;
    let mut bgra = vec![0u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let i = (y * w + x) * 4;
            bgra[i] = u8::try_from(x * 4).unwrap_or(255);
            bgra[i + 1] = u8::try_from(y * 4).unwrap_or(255);
            bgra[i + 2] = 200;
            bgra[i + 3] = 255;
        }
    }

    let png = encode_bgra_to_bytes(&bgra, w, h, ImageFormat::Png)?;
    println!("encoded {} x {} BGRA -> {} bytes of PNG", w, h, png.len());
    assert!(png.starts_with(&[0x89, b'P', b'N', b'G']));

    let decoded = decode_bgra_from_bytes(&png)?;
    println!(
        "decoded back: {}x{} ({} bytes BGRA)",
        decoded.width,
        decoded.height,
        decoded.bgra.len()
    );
    assert_eq!(decoded.width, w);
    assert_eq!(decoded.height, h);
    Ok(())
}
