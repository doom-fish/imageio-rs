//! High-level image read / write / convert API on top of `CGImageSource`
//! and `CGImageDestination`.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use crate::error::ImageError;
use crate::ffi;

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
        return Err(ImageError::OpenSourceFailed("CGImageSourceCreateWithURL returned NULL".into()));
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
        return Err(ImageError::DecodeFailed("CopyPropertiesAtIndex returned NULL".into()));
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
        return Err(ImageError::OpenSourceFailed("CGImageSourceCreateWithURL returned NULL".into()));
    }
    if unsafe { ffi::CGImageSourceGetCount(src) } == 0 {
        unsafe { ffi::CFRelease(src.cast_const()) };
        return Err(ImageError::NoImagesInSource);
    }
    let cg_image = unsafe { ffi::CGImageSourceCreateImageAtIndex(src, 0, ptr::null()) };
    unsafe { ffi::CFRelease(src.cast_const()) };
    if cg_image.is_null() {
        return Err(ImageError::DecodeFailed("CGImageSourceCreateImageAtIndex returned NULL".into()));
    }

    let width = unsafe { ffi::CGImageGetWidth(cg_image) };
    let height = unsafe { ffi::CGImageGetHeight(cg_image) };
    let bytes_per_row = width * 4;
    let mut bgra = vec![0u8; bytes_per_row * height];

    let cs = unsafe { ffi::CGColorSpaceCreateDeviceRGB() };
    let ctx = unsafe {
        ffi::CGBitmapContextCreate(
            bgra.as_mut_ptr().cast(),
            width,
            height,
            8,
            bytes_per_row,
            cs,
            ffi::kCGImageAlphaPremultipliedLast | ffi::kCGBitmapByteOrder32Big,
        )
    };
    unsafe { ffi::CGColorSpaceRelease(cs) };
    if ctx.is_null() {
        unsafe { ffi::CGImageRelease(cg_image) };
        return Err(ImageError::DecodeFailed("CGBitmapContextCreate returned NULL".into()));
    }
    let rect = ffi::CGRect {
        origin: ffi::CGPoint { x: 0.0, y: 0.0 },
        size: ffi::CGSize {
            #[allow(clippy::cast_precision_loss)]
            width: width as f64,
            #[allow(clippy::cast_precision_loss)]
            height: height as f64,
        },
    };
    unsafe { ffi::CGContextDrawImage(ctx, rect, cg_image) };
    unsafe { ffi::CGContextRelease(ctx) };
    unsafe { ffi::CGImageRelease(cg_image) };

    Ok(DecodedImage { width, height, bgra })
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
    unsafe { ffi::CFRelease(in_url) };
    if src.is_null() {
        return Err(ImageError::OpenSourceFailed("CGImageSourceCreateWithURL returned NULL".into()));
    }
    if unsafe { ffi::CGImageSourceGetCount(src) } == 0 {
        unsafe { ffi::CFRelease(src.cast_const()) };
        return Err(ImageError::NoImagesInSource);
    }
    let cg_image = unsafe { ffi::CGImageSourceCreateImageAtIndex(src, 0, ptr::null()) };
    unsafe { ffi::CFRelease(src.cast_const()) };
    if cg_image.is_null() {
        return Err(ImageError::DecodeFailed("CGImageSourceCreateImageAtIndex returned NULL".into()));
    }

    let out_url = make_file_url(output.as_ref())?;
    let uti = make_cf_string(format.as_uti())?;
    let dst = unsafe { ffi::CGImageDestinationCreateWithURL(out_url, uti, 1, ptr::null()) };
    unsafe { ffi::CFRelease(uti) };
    unsafe { ffi::CFRelease(out_url) };
    if dst.is_null() {
        unsafe { ffi::CGImageRelease(cg_image) };
        return Err(ImageError::EncodeFailed("CGImageDestinationCreateWithURL returned NULL".into()));
    }
    unsafe { ffi::CGImageDestinationAddImage(dst, cg_image, ptr::null()) };
    let ok = unsafe { ffi::CGImageDestinationFinalize(dst) };
    unsafe { ffi::CFRelease(dst.cast_const()) };
    unsafe { ffi::CGImageRelease(cg_image) };
    if !ok {
        return Err(ImageError::EncodeFailed("CGImageDestinationFinalize returned false".into()));
    }
    Ok(())
}

// ---- internal helpers ----

fn make_file_url(path: &Path) -> Result<ffi::CFURLRef, ImageError> {
    let s = path
        .to_str()
        .ok_or_else(|| ImageError::InvalidPath("non-UTF-8 path".into()))?;
    let bytes = s.as_bytes();
    let url = unsafe {
        ffi::CFURLCreateFromFileSystemRepresentation(
            ffi::kCFAllocatorDefault,
            bytes.as_ptr(),
            ffi::CFIndex::try_from(bytes.len()).unwrap_or(0),
            false,
        )
    };
    if url.is_null() {
        return Err(ImageError::InvalidPath(format!("CFURL creation failed for {s}")));
    }
    Ok(url)
}

fn make_cf_string(s: &str) -> Result<ffi::CFStringRef, ImageError> {
    let c =
        CString::new(s).map_err(|e| ImageError::Unknown(format!("CString: {e}")))?;
    let cf = unsafe {
        ffi::CFStringCreateWithCString(ffi::kCFAllocatorDefault, c.as_ptr(), ffi::kCFStringEncodingUTF8)
    };
    if cf.is_null() {
        return Err(ImageError::Unknown("CFStringCreateWithCString returned NULL".into()));
    }
    Ok(cf)
}

fn cf_string_to_string(s: ffi::CFStringRef) -> Option<String> {
    if s.is_null() {
        return None;
    }
    let len = unsafe { ffi::CFStringGetLength(s) };
    let cap = len * 4 + 1;
    let mut buf = vec![0u8; usize::try_from(cap).unwrap_or(0)];
    let ok = unsafe {
        ffi::CFStringGetCString(
            s,
            buf.as_mut_ptr().cast::<c_char>(),
            cap,
            ffi::kCFStringEncodingUTF8,
        )
    };
    if !ok {
        return None;
    }
    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }
    String::from_utf8(buf).ok()
}

fn read_dict_int(d: ffi::CFDictionaryRef, key: ffi::CFStringRef) -> Option<i64> {
    let v = unsafe { ffi::CFDictionaryGetValue(d, key.cast()) };
    if v.is_null() {
        return None;
    }
    let mut out: i64 = 0;
    let ok = unsafe { ffi::CFNumberGetValue(v, ffi::kCFNumberSInt64Type, ptr::from_mut(&mut out).cast::<c_void>()) };
    if ok {
        Some(out)
    } else {
        None
    }
}
