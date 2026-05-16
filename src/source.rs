//! Safe wrapper around `CGImageSource`.

use core::ptr;
use std::path::Path;

use crate::error::ImageError;
use crate::ffi;
use crate::image::DecodedImage;
use crate::metadata::Metadata;
use crate::util::{cf_string_to_string, cg_image_to_bgra, make_cf_data, make_file_url};

/// Incremental / file-backed image source status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SourceStatus {
    UnexpectedEof,
    InvalidData,
    UnknownType,
    ReadingHeader,
    Incomplete,
    Complete,
    Unknown(ffi::CGImageSourceStatus),
}

impl From<ffi::CGImageSourceStatus> for SourceStatus {
    fn from(value: ffi::CGImageSourceStatus) -> Self {
        match value {
            ffi::kCGImageStatusUnexpectedEOF => Self::UnexpectedEof,
            ffi::kCGImageStatusInvalidData => Self::InvalidData,
            ffi::kCGImageStatusUnknownType => Self::UnknownType,
            ffi::kCGImageStatusReadingHeader => Self::ReadingHeader,
            ffi::kCGImageStatusIncomplete => Self::Incomplete,
            ffi::kCGImageStatusComplete => Self::Complete,
            other => Self::Unknown(other),
        }
    }
}

/// Safe owning wrapper for `CGImageSourceRef`.
#[derive(Debug)]
pub struct ImageSource {
    raw: ffi::CGImageSourceRef,
}

impl ImageSource {
    fn from_raw(raw: ffi::CGImageSourceRef) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    /// Open an image source from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        let url = make_file_url(path.as_ref())?;
        let source = unsafe { ffi::CGImageSourceCreateWithURL(url, ptr::null()) };
        unsafe { ffi::CFRelease(url.cast()) };
        Self::from_raw(source).ok_or_else(|| {
            ImageError::OpenSourceFailed("CGImageSourceCreateWithURL returned NULL".into())
        })
    }

    /// Open an image source from in-memory encoded bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self, ImageError> {
        let data = make_cf_data(data)?;
        let source = unsafe { ffi::CGImageSourceCreateWithData(data, ptr::null()) };
        unsafe { ffi::CFRelease(data.cast()) };
        Self::from_raw(source).ok_or_else(|| {
            ImageError::OpenSourceFailed("CGImageSourceCreateWithData returned NULL".into())
        })
    }

    /// Create an empty incremental source.
    pub fn incremental() -> Result<Self, ImageError> {
        let source = unsafe { ffi::CGImageSourceCreateIncremental(ptr::null()) };
        Self::from_raw(source).ok_or_else(|| {
            ImageError::OpenSourceFailed("CGImageSourceCreateIncremental returned NULL".into())
        })
    }

    /// Source UTI / type identifier.
    #[must_use]
    pub fn source_type(&self) -> Option<String> {
        cf_string_to_string(unsafe { ffi::CGImageSourceGetType(self.raw) })
    }

    /// Number of images / frames in the source.
    #[must_use]
    pub fn frame_count(&self) -> usize {
        unsafe { ffi::CGImageSourceGetCount(self.raw) }
    }

    /// Source status.
    #[must_use]
    pub fn status(&self) -> SourceStatus {
        unsafe { ffi::CGImageSourceGetStatus(self.raw).into() }
    }

    /// Per-frame status.
    #[must_use]
    pub fn status_at_index(&self, index: usize) -> SourceStatus {
        unsafe { ffi::CGImageSourceGetStatusAtIndex(self.raw, index).into() }
    }

    /// Update an incremental source with more encoded bytes.
    pub fn update_data(&mut self, data: &[u8], is_final: bool) -> Result<(), ImageError> {
        let data = make_cf_data(data)?;
        unsafe {
            ffi::CGImageSourceUpdateData(self.raw, data, is_final);
            ffi::CFRelease(data.cast());
        }
        Ok(())
    }

    /// Decode a frame into tightly packed BGRA bytes.
    pub fn decode_image_at_index(&self, index: usize) -> Result<DecodedImage, ImageError> {
        let image = unsafe { ffi::CGImageSourceCreateImageAtIndex(self.raw, index, ptr::null()) };
        if image.is_null() {
            return Err(ImageError::DecodeFailed(
                "CGImageSourceCreateImageAtIndex returned NULL".into(),
            ));
        }
        let decoded = cg_image_to_bgra(image);
        unsafe { ffi::CGImageRelease(image) };
        decoded
    }

    /// Copy the frame metadata at `index`.
    #[must_use]
    pub fn metadata_at_index(&self, index: usize) -> Option<Metadata> {
        let metadata =
            unsafe { ffi::CGImageSourceCopyMetadataAtIndex(self.raw, index, ptr::null()) };
        Metadata::from_raw(metadata)
    }

    /// Primary image index for formats that designate one.
    #[must_use]
    pub fn primary_image_index(&self) -> usize {
        unsafe { ffi::CGImageSourceGetPrimaryImageIndex(self.raw) }
    }

    /// Clear cached decode state for a frame.
    pub fn remove_cache_at_index(&self, index: usize) {
        unsafe { ffi::CGImageSourceRemoveCacheAtIndex(self.raw, index) };
    }

    #[must_use]
    pub const fn as_raw(&self) -> ffi::CGImageSourceRef {
        self.raw
    }
}

impl Clone for ImageSource {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::CFRetain(self.raw.cast()).cast_mut() };
        Self { raw }
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.raw.cast()) };
    }
}
