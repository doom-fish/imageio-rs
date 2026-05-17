mod common;

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

#[cfg(feature = "raw-ffi")]
#[test]
fn destination_round_trips_via_add_cg_image() {
    // 1. Encode a sample image so we have something to read back as a CGImage
    let image = common::sample_image();
    let mut src_destination = ImageDestination::to_data(ImageFormat::Png.type_identifier(), 1)
        .expect("create png destination");
    src_destination.add_image(&image, None).expect("add seed");
    src_destination.finalize().expect("finalize seed");
    let seed_bytes = src_destination.data().expect("seed data");

    // 2. Open it via raw ImageIO FFI to obtain a CGImage handle (real Apple
    //    CGImage, not a hand-built one — exercises the same zero-copy
    //    integration path that screencapturekit-rs callers will use)
    use imageio::ffi;
    let cg_image: ffi::CGImageRef = unsafe {
        let cf_data = ffi::CFDataCreate(
            std::ptr::null_mut(),
            seed_bytes.as_ptr(),
            seed_bytes.len() as isize,
        );
        assert!(!cf_data.is_null(), "CFDataCreate returned null");
        let isrc = ffi::CGImageSourceCreateWithData(cf_data, std::ptr::null_mut());
        assert!(!isrc.is_null(), "CGImageSourceCreateWithData returned null");
        let img = ffi::CGImageSourceCreateImageAtIndex(isrc, 0, std::ptr::null_mut());
        assert!(!img.is_null(), "CGImageSourceCreateImageAtIndex returned null");
        ffi::CFRelease(isrc as *const _);
        ffi::CFRelease(cf_data as *const _);
        img
    };

    // 3. Pipe the CGImage handle through add_cg_image into a fresh JPEG
    //    destination — no decode-encode round trip, no host-side copy.
    let mut destination = ImageDestination::to_data(ImageFormat::Jpeg.type_identifier(), 1)
        .expect("create jpeg destination");
    unsafe {
        destination
            .add_cg_image(cg_image, None)
            .expect("add_cg_image");
    }
    destination.finalize().expect("finalize");
    let bytes = destination.data().expect("destination data");
    assert!(bytes.len() > 0, "encoded bytes should be non-empty");

    // 4. Decode the result and confirm it's a valid image
    let reread = ImageSource::from_bytes(&bytes).expect("open re-encoded bytes");
    assert!(reread.frame_count() >= 1);
    assert!(reread.source_type().is_some(), "re-encoded JPEG should have a known type");

    unsafe { ffi::CGImageRelease(cg_image) };
}

#[test]
fn destination_add_cg_image_rejects_null() {
    let mut destination = ImageDestination::to_data(ImageFormat::Jpeg.type_identifier(), 1)
        .expect("create destination");
    let err = unsafe { destination.add_cg_image(std::ptr::null_mut(), None) }
        .expect_err("null CGImageRef must error");
    let msg = format!("{err}");
    assert!(msg.contains("null") || msg.to_lowercase().contains("cgimage"), "got: {msg}");
}
