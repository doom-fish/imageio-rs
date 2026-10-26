use imageio::{decode_rgba8_from_bytes, encode_pixels_to_bytes, ImageFormat, PixelFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pixels: Vec<_> = (0..=255).flat_map(|alpha| [201, 103, 47, alpha]).collect();
    let png = encode_pixels_to_bytes(&pixels, 256, 1, PixelFormat::Rgba8, ImageFormat::Png)?;
    let decoded = decode_rgba8_from_bytes(&png)?;
    assert_eq!((decoded.width, decoded.height), (256, 1));
    assert_eq!(decoded.rgba, pixels);
    let rgb: Vec<_> = pixels
        .chunks_exact(4)
        .flat_map(|pixel| pixel[..3].iter().copied())
        .collect();
    let png = encode_pixels_to_bytes(&rgb, 256, 1, PixelFormat::Rgb8, ImageFormat::Png)?;
    let decoded = decode_rgba8_from_bytes(&png)?;
    assert!(decoded
        .rgba
        .chunks_exact(4)
        .all(|pixel| pixel == [201, 103, 47, 255]));
    assert!(decode_rgba8_from_bytes(b"invalid").is_err());
    println!("RGB8, RGBA8, all alpha values, and invalid input: pass");
    for path in std::env::args().skip(1) {
        let decoded = decode_rgba8_from_bytes(&std::fs::read(&path)?)?;
        let checksum = decoded
            .rgba
            .iter()
            .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
                (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
            });
        println!("{}x{} {checksum:016x}", decoded.width, decoded.height);
        std::fs::write(format!("{path}.rgba"), &decoded.rgba)?;
        std::fs::write(
            format!("{path}.roundtrip.png"),
            encode_pixels_to_bytes(
                &decoded.rgba,
                decoded.width,
                decoded.height,
                PixelFormat::Rgba8,
                ImageFormat::Png,
            )?,
        )?;
        let rgb: Vec<_> = decoded
            .rgba
            .chunks_exact(4)
            .flat_map(|pixel| pixel[..3].iter().copied())
            .collect();
        let png = encode_pixels_to_bytes(
            &rgb,
            decoded.width,
            decoded.height,
            PixelFormat::Rgb8,
            ImageFormat::Png,
        )?;
        println!("RGB8 PNG bytes: {}", png.len());
        std::fs::write(format!("{path}.rgb.png"), png)?;
    }
    Ok(())
}
