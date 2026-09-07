mod common;

use std::fs;

use imageio::prelude::*;

#[test]
fn destination_round_trips_encoded_data() {
    let image = common::sample_image();
    let mut destination = ImageDestination::to_data(ImageFormat::Jpeg.type_identifier(), 1)
        .expect("create jpeg destination");
    destination.add_image(&image, None).expect("add image");
    destination.finalize().expect("finalize destination");
    let bytes = destination.data().expect("destination data");

    let source = ImageSource::from_bytes(&bytes).expect("open encoded bytes");
    assert!(source.frame_count() >= 1);
    assert!(source.source_type().is_some());
}

#[test]
fn destination_add_cg_image_accepts_apple_cf_cgimage() {
    use imageio::destination::CGImage;

    // Decode a real CGImage via CGImageSource. This exercises that the
    // re-export from apple-cf compiles and is the right type — and that
    // ImageSource yields one.
    let seed = common::sample_image();
    let mut seed_dest = ImageDestination::to_data(ImageFormat::Png.type_identifier(), 1)
        .expect("create seed destination");
    seed_dest.add_image(&seed, None).expect("add seed");
    seed_dest.finalize().expect("finalize seed");
    let seed_bytes = seed_dest.data().expect("seed bytes");

    let source = ImageSource::from_bytes(&seed_bytes).expect("open seed");
    // Decode to a CGImage via apple-cf's CGImageSource (bypassing imageio's
    // BGRA round-trip path; the goal is to get a real Apple-produced CGImage
    // handle that we can hand to add_cg_image).
    let cg: CGImage = decode_first_cg_image(&seed_bytes);

    let mut dest = ImageDestination::to_data(ImageFormat::Jpeg.type_identifier(), 1)
        .expect("create jpeg destination");
    // ← fully safe API: no `unsafe` block in the consumer call site.
    dest.add_cg_image(&cg, None).expect("add_cg_image");
    dest.finalize().expect("finalize");
    let jpeg = dest.data().expect("jpeg bytes");
    assert!(!jpeg.is_empty(), "encoded JPEG must be non-empty");

    let reread = ImageSource::from_bytes(&jpeg).expect("open re-encoded");
    assert!(reread.frame_count() >= 1);

    // Suppress unused-source warning
    let _ = source;
}

#[test]
fn whole_source_copy_writes_png_and_jpeg_without_finalize() {
    for (format, extension) in [(ImageFormat::Png, "png"), (ImageFormat::Jpeg, "jpg")] {
        let directory = common::work_dir(&format!("whole_source_copy_{extension}"));
        let input = directory.join(format!("input.{extension}"));
        let output = directory.join(format!("output.{extension}"));
        let image = common::sample_image();
        let encoded = encode_bgra_to_bytes(&image.bgra, image.width, image.height, format)
            .expect("encode source image");
        fs::write(&input, encoded).expect("write source image");

        copy_image_source(&input, &output, Some(format)).expect("copy whole image source");

        let copied = ImageSource::from_path(&output).expect("open copied image");
        assert_eq!(copied.frame_count(), 1);
        assert_eq!(copied.source_type(), Some(format.type_identifier().into()));
    }
}

#[test]
fn whole_source_copy_completes_destination_state() {
    for format in [ImageFormat::Png, ImageFormat::Jpeg] {
        let image = common::sample_image();
        let encoded = encode_bgra_to_bytes(&image.bgra, image.width, image.height, format)
            .expect("encode source image");
        let source = ImageSource::from_bytes(&encoded).expect("open source image");
        let mut destination =
            ImageDestination::to_data(format.type_identifier(), 1).expect("create destination");

        destination
            .copy_image_source(&source, None)
            .expect("copy source");
        assert!(
            !destination.data().expect("copied data").is_empty(),
            "whole-source copy must write output immediately"
        );
        assert!(matches!(
            destination.finalize(),
            Err(ImageError::EncodeFailed(message)) if message.contains("already complete")
        ));
        assert!(matches!(
            destination.add_image(&image, None),
            Err(ImageError::EncodeFailed(message)) if message.contains("already complete")
        ));
    }
}

#[test]
fn image_add_paths_share_checked_bgra_layout_validation() {
    let metadata = common::sample_metadata();
    let invalid_images = [
        DecodedImage {
            width: 0,
            height: 1,
            bgra: Vec::new(),
        },
        DecodedImage {
            width: usize::MAX,
            height: 1,
            bgra: Vec::new(),
        },
        DecodedImage {
            width: usize::MAX / 4 + 1,
            height: 1,
            bgra: Vec::new(),
        },
        DecodedImage {
            width: 2,
            height: 2,
            bgra: vec![0; 15],
        },
    ];

    for image in invalid_images {
        let mut plain =
            ImageDestination::to_data(ImageFormat::Png.type_identifier(), 1).expect("destination");
        let plain_error = plain
            .add_image(&image, None)
            .expect_err("plain add must fail");

        let mut with_metadata =
            ImageDestination::to_data(ImageFormat::Png.type_identifier(), 1).expect("destination");
        let metadata_error = with_metadata
            .add_image_with_metadata(&image, &metadata, None)
            .expect_err("metadata add must fail");

        assert_eq!(plain_error, metadata_error);
    }
}

/// Decode the first frame of `bytes` into an `apple-cf` `CGImage` via the
/// `CGImageSource` C API directly. Used only to construct a real Apple
/// `CGImage` for `add_cg_image` round-trip tests.
fn decode_first_cg_image(bytes: &[u8]) -> apple_cf::cg::CGImage {
    use core::ffi::c_void;
    extern "C" {
        fn CFDataCreate(allocator: *const c_void, bytes: *const u8, length: isize) -> *mut c_void;
        fn CFRelease(cf: *const c_void);
        fn CGImageSourceCreateWithData(data: *mut c_void, options: *const c_void) -> *mut c_void;
        fn CGImageSourceCreateImageAtIndex(
            isrc: *mut c_void,
            index: usize,
            options: *const c_void,
        ) -> *mut c_void;
    }
    unsafe {
        let data = CFDataCreate(
            std::ptr::null(),
            bytes.as_ptr(),
            isize::try_from(bytes.len()).unwrap_or(0),
        );
        assert!(!data.is_null());
        let src = CGImageSourceCreateWithData(data, std::ptr::null());
        assert!(!src.is_null());
        let img = CGImageSourceCreateImageAtIndex(src, 0, std::ptr::null());
        assert!(!img.is_null());
        CFRelease(src);
        CFRelease(data);
        apple_cf::cg::CGImage::from_raw(img)
    }
}
