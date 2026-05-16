//! Internal CoreFoundation / CoreGraphics helpers shared across modules.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::path::Path;

use crate::error::ImageError;
use crate::ffi;
use crate::image::DecodedImage;

pub fn make_file_url(path: &Path) -> Result<ffi::CFURLRef, ImageError> {
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
        return Err(ImageError::InvalidPath(format!(
            "CFURL creation failed for {s}"
        )));
    }
    Ok(url)
}

pub fn make_cf_string(s: &str) -> Result<ffi::CFStringRef, ImageError> {
    let c = CString::new(s).map_err(|e| ImageError::Unknown(format!("CString: {e}")))?;
    let cf = unsafe {
        ffi::CFStringCreateWithCString(
            ffi::kCFAllocatorDefault,
            c.as_ptr(),
            ffi::kCFStringEncodingUTF8,
        )
    };
    if cf.is_null() {
        return Err(ImageError::Unknown(
            "CFStringCreateWithCString returned NULL".into(),
        ));
    }
    Ok(cf)
}

pub fn make_cf_data(bytes: &[u8]) -> Result<ffi::CFDataRef, ImageError> {
    let cf = unsafe {
        ffi::CFDataCreate(
            ffi::kCFAllocatorDefault,
            bytes.as_ptr(),
            ffi::CFIndex::try_from(bytes.len()).unwrap_or(0),
        )
    };
    if cf.is_null() {
        return Err(ImageError::Unknown("CFDataCreate returned NULL".into()));
    }
    Ok(cf)
}

pub fn cf_string_to_string(s: ffi::CFStringRef) -> Option<String> {
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

pub fn cf_type_to_string(value: ffi::CFTypeRef) -> Option<String> {
    if value.is_null() {
        return None;
    }
    let is_string = unsafe { ffi::CFGetTypeID(value) == ffi::CFStringGetTypeID() };
    if !is_string {
        return None;
    }
    cf_string_to_string(value.cast())
}

pub fn cf_type_to_i64(value: ffi::CFTypeRef) -> Option<i64> {
    if value.is_null() {
        return None;
    }
    let type_id = unsafe { ffi::CFGetTypeID(value) };
    if type_id == unsafe { ffi::CFNumberGetTypeID() } {
        let mut out: i64 = 0;
        let ok = unsafe {
            ffi::CFNumberGetValue(
                value.cast(),
                ffi::kCFNumberSInt64Type,
                ptr::from_mut(&mut out).cast::<c_void>(),
            )
        };
        return ok.then_some(out);
    }
    if type_id == unsafe { ffi::CFBooleanGetTypeID() } {
        let value = unsafe { ffi::CFBooleanGetValue(value.cast()) };
        return Some(i64::from(value));
    }
    None
}

pub fn cf_data_to_vec(data: ffi::CFDataRef) -> Vec<u8> {
    if data.is_null() {
        return Vec::new();
    }
    let len = unsafe { ffi::CFDataGetLength(data) };
    let len_usize = usize::try_from(len).unwrap_or(0);
    let mut buf = vec![0u8; len_usize];
    if len_usize > 0 {
        unsafe {
            ffi::CFDataGetBytes(
                data,
                ffi::CFRange {
                    location: 0,
                    length: len,
                },
                buf.as_mut_ptr(),
            );
        }
    }
    buf
}

pub fn cf_error_description(error: ffi::CFErrorRef) -> Option<String> {
    if error.is_null() {
        return None;
    }
    let description = unsafe { ffi::CFErrorCopyDescription(error) };
    let message = cf_string_to_string(description);
    if !description.is_null() {
        unsafe { ffi::CFRelease(description.cast()) };
    }
    message
}

pub fn read_dict_int(d: ffi::CFDictionaryRef, key: ffi::CFStringRef) -> Option<i64> {
    let value = unsafe { ffi::CFDictionaryGetValue(d, key.cast()) };
    cf_type_to_i64(value.cast())
}

pub fn cg_image_to_bgra(cg_image: ffi::CGImageRef) -> Result<DecodedImage, ImageError> {
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
        return Err(ImageError::DecodeFailed(
            "CGBitmapContextCreate returned NULL".into(),
        ));
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
    Ok(DecodedImage {
        width,
        height,
        bgra,
    })
}
