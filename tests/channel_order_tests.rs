mod common;

use imageio::prelude::*;

/// Reads the top-left pixel of a baseline uncompressed TIFF as `(r, g, b)`.
///
/// TIFF is used because `ImageIO` writes it uncompressed, so the assertion rests
/// on Apple's encoder alone rather than on this crate's own decode path — a
/// round-trip through `decode_bgra` agrees with `encode_bgra_to_bytes` even
/// when both interpret the buffer with the same wrong channel order.
fn tiff_top_left_rgb(bytes: &[u8]) -> (u8, u8, u8) {
    let big_endian = match &bytes[..2] {
        b"MM" => true,
        b"II" => false,
        other => panic!("not a TIFF: {other:?}"),
    };
    let u16_at = |i: usize| {
        let raw = [bytes[i], bytes[i + 1]];
        if big_endian {
            u16::from_be_bytes(raw)
        } else {
            u16::from_le_bytes(raw)
        }
    };
    let u32_at = |i: usize| {
        let raw = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
        if big_endian {
            u32::from_be_bytes(raw)
        } else {
            u32::from_le_bytes(raw)
        }
    };

    let ifd = u32_at(4) as usize;
    let mut compression = None;
    let mut strip_offset = None;
    for entry in 0..u16_at(ifd) as usize {
        let at = ifd + 2 + entry * 12;
        // A SHORT that fits inline sits in the first two bytes of the value
        // field, which on big-endian TIFF is the high half of the word.
        let value = match u16_at(at + 2) {
            3 => u32::from(u16_at(at + 8)),
            _ => u32_at(at + 8),
        };
        match u16_at(at) {
            259 => compression = Some(value),
            273 => strip_offset = Some(value as usize),
            _ => {}
        }
    }

    assert_eq!(compression, Some(1), "expected an uncompressed TIFF");
    let pixel = strip_offset.expect("TIFF has no StripOffsets tag");
    (bytes[pixel], bytes[pixel + 1], bytes[pixel + 2])
}

fn encode_tiff(image: &DecodedImage) -> Vec<u8> {
    let path = common::work_dir("channel-order").join("probe.tiff");
    let mut destination = ImageDestination::to_path(&path, ImageFormat::Tiff.type_identifier(), 1)
        .expect("create tiff destination");
    destination.add_image(image, None).expect("add image");
    destination.finalize().expect("finalize destination");
    std::fs::read(&path).expect("read tiff")
}

#[test]
fn encode_bgra_treats_the_first_byte_as_blue() {
    let image = DecodedImage {
        width: 1,
        height: 1,
        // Opaque red in BGRA byte order.
        bgra: vec![0, 0, 255, 255],
    };

    assert_eq!(
        tiff_top_left_rgb(&encode_tiff(&image)),
        (255, 0, 0),
        "BGRA 00 00 FF FF is red; encoding it as blue means the channels are swapped"
    );
}

#[test]
fn decode_bgra_round_trips_through_png() {
    let bgra = vec![0, 0, 255, 255];
    let png = encode_bgra_to_bytes(&bgra, 1, 1, ImageFormat::Png).expect("encode png");
    let decoded = decode_bgra_from_bytes(&png).expect("decode png");

    assert_eq!(
        decoded.bgra[..4],
        bgra[..],
        "decoded pixel must stay BGRA red"
    );
}

#[test]
fn sample_image_first_pixel_encodes_as_red() {
    let image = common::sample_image();

    assert_eq!(
        tiff_top_left_rgb(&encode_tiff(&image)),
        (255, 0, 0),
        "the sample image documents its first pixel as red"
    );
}
