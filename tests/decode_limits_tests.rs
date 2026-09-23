use imageio::prelude::*;

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                0xEDB8_8320 ^ (crc >> 1)
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn chunk(kind: [u8; 4], data: &[u8]) -> Vec<u8> {
    let mut body = kind.to_vec();
    body.extend_from_slice(data);
    let mut out = u32::try_from(data.len())
        .expect("chunk fits in u32")
        .to_be_bytes()
        .to_vec();
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
    out
}

fn png_declaring(width: u32, height: u32, padding: usize) -> Vec<u8> {
    let mut header = width.to_be_bytes().to_vec();
    header.extend_from_slice(&height.to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    png.extend(chunk(*b"IHDR", &header));
    if padding > 0 {
        png.extend(chunk(*b"prVt", &vec![0; padding]));
    }
    png.extend(chunk(
        *b"IDAT",
        &[
            0x78, 0x01, 0x01, 0x05, 0x00, 0xfa, 0xff, 0, 0, 0, 0, 0, 0x00, 0x05, 0x00, 0x01,
        ],
    ));
    png.extend(chunk(*b"IEND", &[]));
    png
}

#[test]
fn declared_width_over_the_limit_is_refused_before_decoding() {
    let png = png_declaring(20_000, 8, 0);
    assert!(png.len() < 100);
    assert_eq!(
        decode_bgra_from_bytes(&png),
        Err(ImageError::LimitExceeded {
            width: 20_000,
            height: 8,
            limits: DecodeLimits::default(),
        })
    );
}

#[test]
fn decode_limits_are_configurable_per_source() {
    let png = png_declaring(20_000, 8, 0);
    let mut source = ImageSource::from_bytes(&png).expect("open crafted PNG");
    assert_eq!(source.decode_limits(), DecodeLimits::default());

    let limits = DecodeLimits::new(20_000, 8, 20_000 * 8 * 4);
    source.set_decode_limits(limits);
    assert_eq!(source.clone().decode_limits(), limits);
    let image = source
        .decode_image_at_index(0)
        .expect("decode within raised limits");
    assert_eq!((image.width, image.height), (20_000, 8));
    assert_eq!(image.bgra.len(), 20_000 * 8 * 4);

    source.set_decode_limits(DecodeLimits::new(20_000, 8, 20_000 * 8 * 4 - 1));
    assert!(matches!(
        source.decode_image_at_index(0),
        Err(ImageError::LimitExceeded {
            width: 20_000,
            height: 8,
            ..
        })
    ));
}

#[test]
fn byte_limit_refuses_a_small_file_declaring_a_huge_image() {
    let png = png_declaring(8_193, 8_193, 100_000);
    assert!(png.len() < 101_000);
    let source = ImageSource::from_bytes(&png).expect("open crafted PNG");
    let refused = Err(ImageError::LimitExceeded {
        width: 8_193,
        height: 8_193,
        limits: DecodeLimits::default(),
    });
    assert_eq!(source.decode_image_at_index(0), refused);

    let mut destination =
        ImageDestination::to_data(ImageFormat::Png.type_identifier(), 1).expect("PNG destination");
    assert_eq!(
        destination.add_image_from_source(&source, 0, None),
        refused.map(|_: DecodedImage| ())
    );
}

#[test]
fn thumbnails_are_capped_by_the_decode_limits() {
    let png = png_declaring(8_193, 8_193, 100_000);
    let mut source = ImageSource::from_bytes(&png).expect("open crafted PNG");
    source.set_decode_limits(DecodeLimits::new(64, 64, 64 * 64 * 4));
    for requested in [0, 256, usize::MAX] {
        let thumbnail = create_thumbnail(&source, 0, ThumbnailOptions::new(requested))
            .expect("thumbnail within the limits");
        assert_eq!((thumbnail.width, thumbnail.height), (64, 64));
        assert_eq!(thumbnail.bgra.len(), 64 * 64 * 4);
    }

    source.set_decode_limits(DecodeLimits::new(64, 64, 3));
    assert!(matches!(
        create_thumbnail(&source, 0, ThumbnailOptions::new(16)),
        Err(ImageError::LimitExceeded { .. })
    ));
}
