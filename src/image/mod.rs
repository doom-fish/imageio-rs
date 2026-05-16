//! High-level image read / write / convert API on top of `CGImageSource`
//! and `CGImageDestination`.

use core::ptr;
use std::path::Path;

use crate::error::ImageError;
use crate::ffi;
use crate::util::{
    cf_data_to_vec, cf_error_description, cf_string_to_string, cg_image_to_bgra, make_cf_data,
    make_cf_string, make_file_url, read_dict_int,
};

/// Output image format identifier (the same UTI strings `ImageIO` uses
/// internally).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageFormat {
    Png,
    Jpeg,
    Heic,
    Tiff,
    Gif,
    Bmp,
}

impl ImageFormat {
    /// Return the UTI (Uniform Type Identifier) string `ImageIO` expects.
    #[must_use]
    pub const fn as_uti(self) -> &'static str {
        match self {
            Self::Png => "public.png",
            Self::Jpeg => "public.jpeg",
            Self::Heic => "public.heic",
            Self::Tiff => "public.tiff",
            Self::Gif => "com.compuserve.gif",
            Self::Bmp => "com.microsoft.bmp",
        }
    }
}

/// Decoded image: tightly packed BGRA pixel data plus dimensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedImage {
    pub width: usize,
    pub height: usize,
    /// Tightly packed BGRA bytes, premultiplied alpha. Length =
    /// `width * height * 4`.
    pub bgra: Vec<u8>,
}

impl DecodedImage {
    /// Bytes per row (always `width * 4`).
    #[must_use]
    pub const fn bytes_per_row(&self) -> usize {
        self.width * 4
    }
}

/// Image-source metadata read straight from the file headers (no full
/// decode).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageMetadata {
    pub width: usize,
    pub height: usize,
    /// Number of frames / sub-images in the file (1 for stills, >1 for
    /// animated GIF / multi-page TIFF).
    pub frame_count: usize,
    /// True if the source declares an alpha channel.
    pub has_alpha: bool,
    /// The UTI identifier of the source format (e.g. `"public.png"`).
    pub source_format: Option<String>,
}

/// Read the metadata of the image at `path` without decoding pixels.
///
/// # Errors
///
/// Returns [`ImageError::InvalidPath`] for non-UTF-8 paths,
/// [`ImageError::OpenSourceFailed`] if `ImageIO` can't open the file,
/// or [`ImageError::NoImagesInSource`] for empty containers.
pub fn read_metadata(path: impl AsRef<Path>) -> Result<ImageMetadata, ImageError> {
    let url = make_file_url(path.as_ref())?;
    let src = unsafe { ffi::CGImageSourceCreateWithURL(url, ptr::null()) };
    unsafe { ffi::CFRelease(url) };
    if src.is_null() {
        return Err(ImageError::OpenSourceFailed(
            "CGImageSourceCreateWithURL returned NULL".into(),
        ));
    }
    let result = read_metadata_from_source(src);
    unsafe { ffi::CFRelease(src.cast_const()) };
    result
}

fn read_metadata_from_source(src: ffi::CGImageSourceRef) -> Result<ImageMetadata, ImageError> {
    let count = unsafe { ffi::CGImageSourceGetCount(src) };
    if count == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    let source_type = unsafe { ffi::CGImageSourceGetType(src) };
    let source_format = if source_type.is_null() {
        None
    } else {
        cf_string_to_string(source_type)
    };

    let props = unsafe { ffi::CGImageSourceCopyPropertiesAtIndex(src, 0, ptr::null()) };
    if props.is_null() {
        return Err(ImageError::DecodeFailed(
            "CopyPropertiesAtIndex returned NULL".into(),
        ));
    }
    let width = read_dict_int(props, unsafe { ffi::kCGImagePropertyPixelWidth }).unwrap_or(0);
    let height = read_dict_int(props, unsafe { ffi::kCGImagePropertyPixelHeight }).unwrap_or(0);
    let has_alpha =
        read_dict_int(props, unsafe { ffi::kCGImagePropertyHasAlpha }).is_some_and(|v| v != 0);
    unsafe { ffi::CFRelease(props) };

    Ok(ImageMetadata {
        width: usize::try_from(width).unwrap_or(0),
        height: usize::try_from(height).unwrap_or(0),
        frame_count: count,
        has_alpha,
        source_format,
    })
}

/// Decode the image at `path` into tightly packed BGRA bytes.
///
/// `ImageIO` drives this; supports every format the OS does (PNG, JPEG,
/// HEIC, TIFF, GIF, BMP, RAW, …).
///
/// # Errors
///
/// See [`ImageError`].
pub fn decode_bgra(path: impl AsRef<Path>) -> Result<DecodedImage, ImageError> {
    let url = make_file_url(path.as_ref())?;
    let src = unsafe { ffi::CGImageSourceCreateWithURL(url, ptr::null()) };
    unsafe { ffi::CFRelease(url) };
    if src.is_null() {
        return Err(ImageError::OpenSourceFailed(
            "CGImageSourceCreateWithURL returned NULL".into(),
        ));
    }
    if unsafe { ffi::CGImageSourceGetCount(src) } == 0 {
        unsafe { ffi::CFRelease(src.cast_const()) };
        return Err(ImageError::NoImagesInSource);
    }
    let cg_image = unsafe { ffi::CGImageSourceCreateImageAtIndex(src, 0, ptr::null()) };
    unsafe { ffi::CFRelease(src.cast_const()) };
    if cg_image.is_null() {
        return Err(ImageError::DecodeFailed(
            "CGImageSourceCreateImageAtIndex returned NULL".into(),
        ));
    }

    let decoded = cg_image_to_bgra(cg_image);
    unsafe { ffi::CGImageRelease(cg_image) };
    decoded
}

/// Convert an image file from any ImageIO-readable format to the requested
/// output format. The output `path` extension is informational; the
/// actual encoding is driven by `format`.
///
/// # Errors
///
/// See [`ImageError`].
pub fn convert_format(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: ImageFormat,
) -> Result<(), ImageError> {
    let in_url = make_file_url(input.as_ref())?;
    let src = unsafe { ffi::CGImageSourceCreateWithURL(in_url, ptr::null()) };
    unsafe { ffi::CFRelease(in_url.cast()) };
    if src.is_null() {
        return Err(ImageError::OpenSourceFailed(
            "CGImageSourceCreateWithURL returned NULL".into(),
        ));
    }
    if unsafe { ffi::CGImageSourceGetCount(src) } == 0 {
        unsafe { ffi::CFRelease(src.cast()) };
        return Err(ImageError::NoImagesInSource);
    }
    let cg_image = unsafe { ffi::CGImageSourceCreateImageAtIndex(src, 0, ptr::null()) };
    unsafe { ffi::CFRelease(src.cast()) };
    if cg_image.is_null() {
        return Err(ImageError::DecodeFailed(
            "CGImageSourceCreateImageAtIndex returned NULL".into(),
        ));
    }

    let out_url = make_file_url(output.as_ref())?;
    let uti = make_cf_string(format.as_uti())?;
    let dst = unsafe { ffi::CGImageDestinationCreateWithURL(out_url, uti, 1, ptr::null()) };
    unsafe {
        ffi::CFRelease(uti.cast());
        ffi::CFRelease(out_url.cast());
    }
    if dst.is_null() {
        unsafe { ffi::CGImageRelease(cg_image) };
        return Err(ImageError::EncodeFailed(
            "CGImageDestinationCreateWithURL returned NULL".into(),
        ));
    }
    unsafe { ffi::CGImageDestinationAddImage(dst, cg_image, ptr::null()) };
    let ok = unsafe { ffi::CGImageDestinationFinalize(dst) };
    unsafe {
        ffi::CFRelease(dst.cast());
        ffi::CGImageRelease(cg_image);
    }
    if !ok {
        return Err(ImageError::EncodeFailed(
            "CGImageDestinationFinalize returned false".into(),
        ));
    }
    Ok(())
}

/// Copy an image source to a new destination without forcing a full decode.
///
/// When `format` is `None`, the source type is preserved.
///
/// # Errors
///
/// See [`ImageError`].
pub fn copy_image_source(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: Option<ImageFormat>,
) -> Result<(), ImageError> {
    let in_url = make_file_url(input.as_ref())?;
    let src = unsafe { ffi::CGImageSourceCreateWithURL(in_url, ptr::null()) };
    unsafe { ffi::CFRelease(in_url.cast()) };
    if src.is_null() {
        return Err(ImageError::OpenSourceFailed(
            "CGImageSourceCreateWithURL returned NULL".into(),
        ));
    }
    let count = unsafe { ffi::CGImageSourceGetCount(src) };
    if count == 0 {
        unsafe { ffi::CFRelease(src.cast()) };
        return Err(ImageError::NoImagesInSource);
    }

    let (destination_type, release_destination_type) = if let Some(format) = format {
        (make_cf_string(format.as_uti())?, true)
    } else {
        let ty = unsafe { ffi::CGImageSourceGetType(src) };
        if ty.is_null() {
            unsafe { ffi::CFRelease(src.cast()) };
            return Err(ImageError::UnsupportedFormat(
                "CGImageSourceGetType returned NULL".into(),
            ));
        }
        (ty, false)
    };

    let out_url = make_file_url(output.as_ref())?;
    let dst = unsafe {
        ffi::CGImageDestinationCreateWithURL(out_url, destination_type, count, ptr::null())
    };
    unsafe { ffi::CFRelease(out_url.cast()) };
    if release_destination_type {
        unsafe { ffi::CFRelease(destination_type.cast()) };
    }
    if dst.is_null() {
        unsafe { ffi::CFRelease(src.cast()) };
        return Err(ImageError::EncodeFailed(
            "CGImageDestinationCreateWithURL returned NULL".into(),
        ));
    }

    let mut error: ffi::CFErrorRef = ptr::null();
    let ok = unsafe {
        ffi::CGImageDestinationCopyImageSource(dst, src, ptr::null(), ptr::from_mut(&mut error))
    };
    unsafe {
        ffi::CFRelease(dst.cast());
        ffi::CFRelease(src.cast());
    }
    if ok {
        return Ok(());
    }
    let message = cf_error_description(error)
        .unwrap_or_else(|| "CGImageDestinationCopyImageSource returned false".into());
    if !error.is_null() {
        unsafe { ffi::CFRelease(error.cast()) };
    }
    Err(ImageError::EncodeFailed(message))
}

/// Decode an image already in memory (no file I/O) into a 32-bpp
/// BGRA buffer with premultiplied alpha.
///
/// `data` may be PNG, JPEG, HEIC, TIFF, GIF, BMP, or any other format
/// `ImageIO` supports.
///
/// # Errors
///
/// See [`ImageError`].
pub fn decode_bgra_from_bytes(data: &[u8]) -> Result<DecodedImage, ImageError> {
    let cfdata = make_cf_data(data)?;
    let src = unsafe { ffi::CGImageSourceCreateWithData(cfdata, ptr::null()) };
    unsafe { ffi::CFRelease(cfdata) };
    if src.is_null() {
        return Err(ImageError::OpenSourceFailed(
            "CGImageSourceCreateWithData returned NULL".into(),
        ));
    }
    if unsafe { ffi::CGImageSourceGetCount(src) } == 0 {
        unsafe { ffi::CFRelease(src.cast_const()) };
        return Err(ImageError::NoImagesInSource);
    }
    let cg_image = unsafe { ffi::CGImageSourceCreateImageAtIndex(src, 0, ptr::null()) };
    unsafe { ffi::CFRelease(src.cast_const()) };
    if cg_image.is_null() {
        return Err(ImageError::DecodeFailed(
            "CGImageSourceCreateImageAtIndex returned NULL".into(),
        ));
    }
    let result = cg_image_to_bgra(cg_image);
    unsafe { ffi::CGImageRelease(cg_image) };
    result
}

/// Encode raw BGRA bytes (premultiplied alpha) to an in-memory
/// byte buffer in the requested `format`.
///
/// # Errors
///
/// See [`ImageError`].
pub fn encode_bgra_to_bytes(
    bgra: &[u8],
    width: usize,
    height: usize,
    format: ImageFormat,
) -> Result<Vec<u8>, ImageError> {
    if bgra.len() < width * height * 4 {
        return Err(ImageError::InvalidPath(format!(
            "buffer too small for {width}x{height} BGRA"
        )));
    }
    // Build a CGImage from the buffer.
    let cs = unsafe { ffi::CGColorSpaceCreateDeviceRGB() };
    let ctx = unsafe {
        ffi::CGBitmapContextCreate(
            bgra.as_ptr().cast::<core::ffi::c_void>().cast_mut(),
            width,
            height,
            8,
            width * 4,
            cs,
            ffi::kCGImageAlphaPremultipliedLast | ffi::kCGBitmapByteOrder32Big,
        )
    };
    unsafe { ffi::CGColorSpaceRelease(cs) };
    if ctx.is_null() {
        return Err(ImageError::EncodeFailed(
            "CGBitmapContextCreate returned NULL".into(),
        ));
    }
    let cg_image = unsafe { ffi::CGBitmapContextCreateImage(ctx) };
    unsafe { ffi::CGContextRelease(ctx) };
    if cg_image.is_null() {
        return Err(ImageError::EncodeFailed(
            "CGBitmapContextCreateImage returned NULL".into(),
        ));
    }

    let cfdata = unsafe { ffi::CFDataCreateMutable(ffi::kCFAllocatorDefault, 0) };
    let uti = make_cf_string(format.as_uti())?;
    let dst = unsafe { ffi::CGImageDestinationCreateWithData(cfdata, uti, 1, ptr::null()) };
    unsafe { ffi::CFRelease(uti) };
    if dst.is_null() {
        unsafe { ffi::CFRelease(cfdata) };
        unsafe { ffi::CGImageRelease(cg_image) };
        return Err(ImageError::EncodeFailed(
            "CGImageDestinationCreateWithData returned NULL".into(),
        ));
    }
    unsafe { ffi::CGImageDestinationAddImage(dst, cg_image, ptr::null()) };
    let ok = unsafe { ffi::CGImageDestinationFinalize(dst) };
    unsafe { ffi::CFRelease(dst.cast_const()) };
    unsafe { ffi::CGImageRelease(cg_image) };
    if !ok {
        unsafe { ffi::CFRelease(cfdata) };
        return Err(ImageError::EncodeFailed(
            "CGImageDestinationFinalize returned false".into(),
        ));
    }

    let buf = cf_data_to_vec(cfdata);
    unsafe { ffi::CFRelease(cfdata) };
    Ok(buf)
}
