//! Raw FFI declarations for the subset of `ImageIO` + CoreGraphics +
//! CoreFoundation we use.
//!
//! `ImageIO` is a pure C framework — no Swift bridge needed. We declare
//! `extern "C"` and link against the system libraries directly.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]

use core::ffi::{c_char, c_void};

pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFURLRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFMutableDataRef = *mut c_void;
pub type CFDataRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFIndex = isize;

pub type CGImageRef = *mut c_void;
pub type CGImageSourceRef = *mut c_void;
pub type CGImageDestinationRef = *mut c_void;
pub type CGContextRef = *mut c_void;
pub type CGColorSpaceRef = *mut c_void;
pub type CGDataProviderRef = *mut c_void;

pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;

// CGBitmapInfo / alpha-info constants.
pub const kCGImageAlphaPremultipliedLast: u32 = 1;
pub const kCGImageAlphaNoneSkipLast: u32 = 5;
pub const kCGBitmapByteOrder32Big: u32 = 4 << 12;

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;

    // CoreFoundation
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetCString(
        s: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: u32,
    ) -> bool;
    pub fn CFStringGetLength(s: CFStringRef) -> CFIndex;
    pub fn CFURLCreateFromFileSystemRepresentation(
        alloc: CFAllocatorRef,
        buffer: *const u8,
        buf_len: CFIndex,
        is_dir: bool,
    ) -> CFURLRef;
    pub fn CFDataCreateMutable(alloc: CFAllocatorRef, capacity: CFIndex) -> CFMutableDataRef;
    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytes(data: CFDataRef, range: CFRange, buffer: *mut u8);
    pub fn CFDictionaryGetValue(d: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn CFNumberGetValue(num: *const c_void, the_type: i64, value_ptr: *mut c_void) -> bool;

    // ImageIO — keys + sources + destinations
    pub static kCGImagePropertyPixelWidth: CFStringRef;
    pub static kCGImagePropertyPixelHeight: CFStringRef;
    pub static kCGImagePropertyHasAlpha: CFStringRef;
    pub static kCGImagePropertyColorModel: CFStringRef;
    pub static kCGImagePropertyDepth: CFStringRef;

    pub fn CGImageSourceCreateWithURL(
        url: CFURLRef,
        options: CFDictionaryRef,
    ) -> CGImageSourceRef;
    pub fn CGImageSourceCreateWithData(
        data: CFDataRef,
        options: CFDictionaryRef,
    ) -> CGImageSourceRef;
    pub fn CGImageSourceGetType(isrc: CGImageSourceRef) -> CFStringRef;
    pub fn CGImageSourceGetCount(isrc: CGImageSourceRef) -> usize;
    pub fn CGImageSourceCopyTypeIdentifiers() -> *const c_void;
    pub fn CGImageSourceCopyProperties(
        isrc: CGImageSourceRef,
        options: CFDictionaryRef,
    ) -> CFDictionaryRef;
    pub fn CGImageSourceCopyPropertiesAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CFDictionaryRef;
    pub fn CGImageSourceCreateImageAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageRef;

    pub fn CGImageDestinationCreateWithURL(
        url: CFURLRef,
        ty: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
    pub fn CGImageDestinationCreateWithData(
        data: CFMutableDataRef,
        ty: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
    pub fn CGImageDestinationCopyTypeIdentifiers() -> *const c_void;
    pub fn CGImageDestinationAddImage(
        idst: CGImageDestinationRef,
        image: CGImageRef,
        properties: CFDictionaryRef,
    );
    pub fn CGImageDestinationFinalize(idst: CGImageDestinationRef) -> bool;

    // CoreGraphics — CGImage + CGContext + CGColorSpace
    pub fn CGImageGetWidth(image: CGImageRef) -> usize;
    pub fn CGImageGetHeight(image: CGImageRef) -> usize;
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
    pub fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
    pub fn CGImageGetAlphaInfo(image: CGImageRef) -> u32;
    pub fn CGImageRelease(image: CGImageRef);

    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
    pub fn CGColorSpaceRelease(cs: CGColorSpaceRef);

    pub fn CGBitmapContextCreate(
        data: *mut c_void,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: u32,
    ) -> CGContextRef;
    pub fn CGContextDrawImage(c: CGContextRef, rect: CGRect, image: CGImageRef);
    pub fn CGContextRelease(c: CGContextRef);

    pub fn CGDataProviderCreateWithData(
        info: *mut c_void,
        data: *const c_void,
        size: usize,
        release_info: Option<extern "C" fn(*mut c_void, *const c_void, usize)>,
    ) -> CGDataProviderRef;
    pub fn CGDataProviderRelease(provider: CGDataProviderRef);
    pub fn CGImageCreate(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: u32,
        provider: CGDataProviderRef,
        decode: *const f64,
        should_interpolate: bool,
        intent: u32,
    ) -> CGImageRef;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CGSize {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

pub const kCFNumberSInt64Type: i64 = 4;
