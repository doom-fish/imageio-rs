//! Raw FFI declarations for Apple's `ImageIO` framework plus the
//! CoreFoundation/CoreGraphics types and functions needed by the safe wrappers.
//!
//! `ImageIO` is a pure C framework, so the full SDK surface is exposed through
//! `extern "C"` declarations linked against the system frameworks directly.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub use apple_cf::cg::{CGPoint, CGRect, CGSize};
pub use apple_cf::raw::{
    CFAllocatorRef, CFArrayRef, CFBooleanRef, CFDataRef, CFDictionaryRef, CFErrorRef, CFIndex,
    CFMutableDataRef, CFNumberRef, CFRange, CFRunLoopRef, CFStringRef, CFTypeID, CFTypeRef,
    CFURLRef, CGContextRef, CGColorSpaceRef, OSStatus,
};

pub type CGImageRef = *mut c_void;
pub type CGImageSourceRef = *mut c_void;
pub type CGImageDestinationRef = *mut c_void;
pub type CGImageMetadataRef = *const c_void;
pub type CGMutableImageMetadataRef = *mut c_void;
pub type CGImageMetadataTagRef = *const c_void;
pub type CGDataProviderRef = *mut c_void;
pub type CGDataConsumerRef = *mut c_void;

pub type CGImageSourceAnimationBlock = *const c_void;
pub type CGImageMetadataTagBlock = *const c_void;

pub type CGImageSourceStatus = i32;
pub const kCGImageStatusUnexpectedEOF: CGImageSourceStatus = -5;
pub const kCGImageStatusInvalidData: CGImageSourceStatus = -4;
pub const kCGImageStatusUnknownType: CGImageSourceStatus = -3;
pub const kCGImageStatusReadingHeader: CGImageSourceStatus = -2;
pub const kCGImageStatusIncomplete: CGImageSourceStatus = -1;
pub const kCGImageStatusComplete: CGImageSourceStatus = 0;

pub type CGImageAnimationStatus = OSStatus;
pub const kCGImageAnimationStatus_ParameterError: CGImageAnimationStatus = -22_140;
pub const kCGImageAnimationStatus_CorruptInputImage: CGImageAnimationStatus = -22_141;
pub const kCGImageAnimationStatus_UnsupportedFormat: CGImageAnimationStatus = -22_142;
pub const kCGImageAnimationStatus_IncompleteInputImage: CGImageAnimationStatus = -22_143;
pub const kCGImageAnimationStatus_AllocationFailure: CGImageAnimationStatus = -22_144;

pub type CGImageMetadataType = i32;
pub const kCGImageMetadataTypeInvalid: CGImageMetadataType = -1;
pub const kCGImageMetadataTypeDefault: CGImageMetadataType = 0;
pub const kCGImageMetadataTypeString: CGImageMetadataType = 1;
pub const kCGImageMetadataTypeArrayUnordered: CGImageMetadataType = 2;
pub const kCGImageMetadataTypeArrayOrdered: CGImageMetadataType = 3;
pub const kCGImageMetadataTypeAlternateArray: CGImageMetadataType = 4;
pub const kCGImageMetadataTypeAlternateText: CGImageMetadataType = 5;
pub const kCGImageMetadataTypeStructure: CGImageMetadataType = 6;

pub type CGImageMetadataErrors = i32;
pub const kCGImageMetadataErrorUnknown: CGImageMetadataErrors = 0;
pub const kCGImageMetadataErrorUnsupportedFormat: CGImageMetadataErrors = 1;
pub const kCGImageMetadataErrorBadArgument: CGImageMetadataErrors = 2;
pub const kCGImageMetadataErrorConflictingArguments: CGImageMetadataErrors = 3;
pub const kCGImageMetadataErrorPrefixConflict: CGImageMetadataErrors = 4;

pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;
pub const kCFNumberSInt64Type: i64 = 4;

// CGBitmapInfo / alpha-info constants.
pub const kCGImageAlphaPremultipliedLast: u32 = 1;
pub const kCGImageAlphaPremultipliedFirst: u32 = 2;
pub const kCGImageAlphaNoneSkipLast: u32 = 5;
pub const kCGBitmapByteOrder32Little: u32 = 2 << 12;
pub const kCGBitmapByteOrder32Big: u32 = 4 << 12;

unsafe extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;
    pub static kCFRunLoopDefaultMode: CFStringRef;

    // CoreFoundation
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFStringGetTypeID() -> CFTypeID;
    pub fn CFBooleanGetTypeID() -> CFTypeID;
    pub fn CFNumberGetTypeID() -> CFTypeID;
    pub fn CFErrorCopyDescription(err: CFErrorRef) -> CFStringRef;
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
    pub fn CFDataCreate(alloc: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytes(data: CFDataRef, range: CFRange, buffer: *mut u8);
    pub fn CFRunLoopRunInMode(
        mode: CFStringRef,
        seconds: f64,
        returnAfterSourceHandled: bool,
    ) -> i32;
    pub fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(array: CFArrayRef, index: CFIndex) -> *const c_void;
    pub fn CFDictionaryGetValue(d: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn CFNumberGetValue(num: CFNumberRef, the_type: i64, value_ptr: *mut c_void) -> bool;
    pub fn CFBooleanGetValue(boolean: CFBooleanRef) -> bool;

    // CGImageSource.h
    pub fn CGImageSourceGetTypeID() -> CFTypeID;
    pub fn CGImageSourceCopyTypeIdentifiers() -> CFArrayRef;
    pub fn CGImageSourceCreateWithDataProvider(
        provider: CGDataProviderRef,
        options: CFDictionaryRef,
    ) -> CGImageSourceRef;
    pub fn CGImageSourceCreateWithData(
        data: CFDataRef,
        options: CFDictionaryRef,
    ) -> CGImageSourceRef;
    pub fn CGImageSourceCreateWithURL(url: CFURLRef, options: CFDictionaryRef) -> CGImageSourceRef;
    pub fn CGImageSourceGetType(isrc: CGImageSourceRef) -> CFStringRef;
    pub fn CGImageSourceGetCount(isrc: CGImageSourceRef) -> usize;
    pub fn CGImageSourceCopyProperties(
        isrc: CGImageSourceRef,
        options: CFDictionaryRef,
    ) -> CFDictionaryRef;
    pub fn CGImageSourceCopyPropertiesAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CFDictionaryRef;
    pub fn CGImageSourceCopyMetadataAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageMetadataRef;
    pub fn CGImageSourceCreateImageAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageRef;
    pub fn CGImageSourceRemoveCacheAtIndex(isrc: CGImageSourceRef, index: usize);
    pub fn CGImageSourceCreateThumbnailAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        options: CFDictionaryRef,
    ) -> CGImageRef;
    pub fn CGImageSourceCreateIncremental(options: CFDictionaryRef) -> CGImageSourceRef;
    pub fn CGImageSourceUpdateData(isrc: CGImageSourceRef, data: CFDataRef, final_data: bool);
    pub fn CGImageSourceUpdateDataProvider(
        isrc: CGImageSourceRef,
        provider: CGDataProviderRef,
        final_data: bool,
    );
    pub fn CGImageSourceGetStatus(isrc: CGImageSourceRef) -> CGImageSourceStatus;
    pub fn CGImageSourceGetStatusAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
    ) -> CGImageSourceStatus;
    pub fn CGImageSourceGetPrimaryImageIndex(isrc: CGImageSourceRef) -> usize;
    pub fn CGImageSourceCopyAuxiliaryDataInfoAtIndex(
        isrc: CGImageSourceRef,
        index: usize,
        auxiliaryImageDataType: CFStringRef,
    ) -> CFDictionaryRef;
    pub fn CGImageSourceSetAllowableTypes(allowableTypes: CFArrayRef) -> OSStatus;

    // CGImageDestination.h
    pub fn CGImageDestinationGetTypeID() -> CFTypeID;
    pub fn CGImageDestinationCopyTypeIdentifiers() -> CFArrayRef;
    pub fn CGImageDestinationCreateWithDataConsumer(
        consumer: CGDataConsumerRef,
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
    pub fn CGImageDestinationCreateWithURL(
        url: CFURLRef,
        ty: CFStringRef,
        count: usize,
        options: CFDictionaryRef,
    ) -> CGImageDestinationRef;
    pub fn CGImageDestinationSetProperties(
        idst: CGImageDestinationRef,
        properties: CFDictionaryRef,
    );
    pub fn CGImageDestinationAddImage(
        idst: CGImageDestinationRef,
        image: CGImageRef,
        properties: CFDictionaryRef,
    );
    pub fn CGImageDestinationAddImageFromSource(
        idst: CGImageDestinationRef,
        isrc: CGImageSourceRef,
        index: usize,
        properties: CFDictionaryRef,
    );
    pub fn CGImageDestinationFinalize(idst: CGImageDestinationRef) -> bool;
    pub fn CGImageDestinationAddImageAndMetadata(
        idst: CGImageDestinationRef,
        image: CGImageRef,
        metadata: CGImageMetadataRef,
        options: CFDictionaryRef,
    );
    pub fn CGImageDestinationCopyImageSource(
        idst: CGImageDestinationRef,
        isrc: CGImageSourceRef,
        options: CFDictionaryRef,
        err: *mut CFErrorRef,
    ) -> bool;
    pub fn CGImageDestinationAddAuxiliaryDataInfo(
        idst: CGImageDestinationRef,
        auxiliaryImageDataType: CFStringRef,
        auxiliaryDataInfoDictionary: CFDictionaryRef,
    );

    // CGImageAnimation.h
    pub fn CGAnimateImageAtURLWithBlock(
        url: CFURLRef,
        options: CFDictionaryRef,
        block: CGImageSourceAnimationBlock,
    ) -> CGImageAnimationStatus;
    pub fn CGAnimateImageDataWithBlock(
        data: CFDataRef,
        options: CFDictionaryRef,
        block: CGImageSourceAnimationBlock,
    ) -> CGImageAnimationStatus;

    // CGImageMetadata.h
    pub fn CGImageMetadataGetTypeID() -> CFTypeID;
    pub fn CGImageMetadataCreateMutable() -> CGMutableImageMetadataRef;
    pub fn CGImageMetadataCreateMutableCopy(
        metadata: CGImageMetadataRef,
    ) -> CGMutableImageMetadataRef;
    pub fn CGImageMetadataTagGetTypeID() -> CFTypeID;
    pub fn CGImageMetadataTagCreate(
        xmlns: CFStringRef,
        prefix: CFStringRef,
        name: CFStringRef,
        tag_type: CGImageMetadataType,
        value: CFTypeRef,
    ) -> CGImageMetadataTagRef;
    pub fn CGImageMetadataTagCopyNamespace(tag: CGImageMetadataTagRef) -> CFStringRef;
    pub fn CGImageMetadataTagCopyPrefix(tag: CGImageMetadataTagRef) -> CFStringRef;
    pub fn CGImageMetadataTagCopyName(tag: CGImageMetadataTagRef) -> CFStringRef;
    pub fn CGImageMetadataTagCopyValue(tag: CGImageMetadataTagRef) -> CFTypeRef;
    pub fn CGImageMetadataTagGetType(tag: CGImageMetadataTagRef) -> CGImageMetadataType;
    pub fn CGImageMetadataTagCopyQualifiers(tag: CGImageMetadataTagRef) -> CFArrayRef;
    pub fn CGImageMetadataCopyTags(metadata: CGImageMetadataRef) -> CFArrayRef;
    pub fn CGImageMetadataCopyTagWithPath(
        metadata: CGImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
    ) -> CGImageMetadataTagRef;
    pub fn CGImageMetadataCopyStringValueWithPath(
        metadata: CGImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
    ) -> CFStringRef;
    pub fn CGImageMetadataRegisterNamespaceForPrefix(
        metadata: CGMutableImageMetadataRef,
        xmlns: CFStringRef,
        prefix: CFStringRef,
        err: *mut CFErrorRef,
    ) -> bool;
    pub fn CGImageMetadataSetTagWithPath(
        metadata: CGMutableImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
        tag: CGImageMetadataTagRef,
    ) -> bool;
    pub fn CGImageMetadataSetValueWithPath(
        metadata: CGMutableImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
        value: CFTypeRef,
    ) -> bool;
    pub fn CGImageMetadataRemoveTagWithPath(
        metadata: CGMutableImageMetadataRef,
        parent: CGImageMetadataTagRef,
        path: CFStringRef,
    ) -> bool;
    pub fn CGImageMetadataEnumerateTagsUsingBlock(
        metadata: CGImageMetadataRef,
        rootPath: CFStringRef,
        options: CFDictionaryRef,
        block: CGImageMetadataTagBlock,
    );
    pub fn CGImageMetadataCopyTagMatchingImageProperty(
        metadata: CGImageMetadataRef,
        dictionaryName: CFStringRef,
        propertyName: CFStringRef,
    ) -> CGImageMetadataTagRef;
    pub fn CGImageMetadataSetValueMatchingImageProperty(
        metadata: CGMutableImageMetadataRef,
        dictionaryName: CFStringRef,
        propertyName: CFStringRef,
        value: CFTypeRef,
    ) -> bool;
    pub fn CGImageMetadataCreateXMPData(
        metadata: CGImageMetadataRef,
        options: CFDictionaryRef,
    ) -> CFDataRef;
    pub fn CGImageMetadataCreateFromXMPData(data: CFDataRef) -> CGImageMetadataRef;

    // CoreGraphics — `CGImage` + `CGContext` + `CGColorSpace`.
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
    pub fn CGBitmapContextCreateImage(context: CGContextRef) -> CGImageRef;

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

include!("generated_constants.rs");
