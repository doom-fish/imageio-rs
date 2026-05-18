//! Safe wrapper around `CGImageSource`.

use std::path::Path;

use crate::auxiliary_data::{AuxiliaryDataInfo, AuxiliaryDataType};
use crate::bridge::{self, source as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::metadata::Metadata;
use crate::properties::ImageProperties;

/// Incremental and file-backed source state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SourceStatus {
    /// Matches `kCGImageStatusUnexpectedEOF`.
    UnexpectedEof,
    /// Matches `kCGImageStatusInvalidData`.
    InvalidData,
    /// Matches `kCGImageStatusUnknownType`.
    UnknownType,
    /// Matches `kCGImageStatusReadingHeader`.
    ReadingHeader,
    /// Matches `kCGImageStatusIncomplete`.
    Incomplete,
    /// Matches `kCGImageStatusComplete`.
    Complete,
    /// Preserves an unknown `CGImageSourceStatus` value.
    Unknown(i32),
}

impl From<i32> for SourceStatus {
    fn from(value: i32) -> Self {
        match value {
            -5 => Self::UnexpectedEof,
            -4 => Self::InvalidData,
            -3 => Self::UnknownType,
            -2 => Self::ReadingHeader,
            -1 => Self::Incomplete,
            0 => Self::Complete,
            other => Self::Unknown(other),
        }
    }
}

/// Owned image source.
#[derive(Debug)]
pub struct ImageSource {
    raw: Handle,
}

impl ImageSource {
    pub(crate) fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> Handle {
        self.raw
    }

    #[must_use]
    /// Wraps `CGImageSourceCopyTypeIdentifiers`.
    pub fn type_identifiers() -> Vec<String> {
        bridge::copy_string_array(unsafe { ffi::imageio_source_copy_type_identifiers() })
    }

    /// Wraps `CGImageSourceCreateWithURL`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        let path = bridge::path_to_cstring(path.as_ref())?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_from_path(path.as_ptr(), buffer, size)
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::OpenSourceFailed(if message.is_empty() {
                "imageio_source_create_from_path returned NULL".into()
            } else {
                message
            })
        })
    }

    /// Wraps `CGImageSourceCreateWithData`.
    pub fn from_bytes(data: &[u8]) -> Result<Self, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_from_bytes(data.as_ptr(), data.len(), buffer, size)
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::OpenSourceFailed(if message.is_empty() {
                "imageio_source_create_from_bytes returned NULL".into()
            } else {
                message
            })
        })
    }

    /// Wraps `CGImageSourceCreateIncremental`.
    pub fn incremental() -> Result<Self, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_incremental(buffer, size)
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::OpenSourceFailed(if message.is_empty() {
                "imageio_source_create_incremental returned NULL".into()
            } else {
                message
            })
        })
    }

    #[must_use]
    /// Wraps `CGImageSourceGetType`.
    pub fn source_type(&self) -> Option<String> {
        bridge::copy_string(unsafe { ffi::imageio_source_copy_type(self.raw) })
    }

    #[must_use]
    /// Wraps `CGImageSourceGetCount`.
    pub fn frame_count(&self) -> usize {
        unsafe { ffi::imageio_source_get_count(self.raw) }
    }

    #[must_use]
    /// Wraps `CGImageSourceGetStatus`.
    pub fn status(&self) -> SourceStatus {
        unsafe { ffi::imageio_source_get_status(self.raw) }.into()
    }

    #[must_use]
    /// Wraps `CGImageSourceGetStatusAtIndex`.
    pub fn status_at_index(&self, index: usize) -> SourceStatus {
        unsafe { ffi::imageio_source_get_status_at_index(self.raw, index) }.into()
    }

    /// Wraps `CGImageSourceUpdateData`.
    pub fn update_data(&mut self, data: &[u8], is_final: bool) -> Result<(), ImageError> {
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_update_data(
                self.raw,
                data.as_ptr(),
                data.len(),
                is_final,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::OpenSourceFailed(if message.is_empty() {
                "imageio_source_update_data returned false".into()
            } else {
                message
            }))
        }
    }

    /// Wraps `CGImageSourceCopyProperties`.
    pub fn copy_properties(&self) -> Result<ImageProperties, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_copy_properties(self.raw, buffer, size)
        });
        ImageProperties::from_raw(raw).ok_or_else(|| {
            ImageError::DecodeFailed(if message.is_empty() {
                "imageio_source_copy_properties returned NULL".into()
            } else {
                message
            })
        })
    }

    /// Wraps `CGImageSourceCopyPropertiesAtIndex`.
    pub fn properties_at_index(&self, index: usize) -> Result<ImageProperties, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_copy_properties_at_index(self.raw, index, buffer, size)
        });
        ImageProperties::from_raw(raw).ok_or_else(|| {
            ImageError::DecodeFailed(if message.is_empty() {
                "imageio_source_copy_properties_at_index returned NULL".into()
            } else {
                message
            })
        })
    }

    #[must_use]
    /// Wraps `CGImageSourceCopyMetadataAtIndex`.
    pub fn metadata_at_index(&self, index: usize) -> Option<Metadata> {
        Metadata::from_raw(unsafe { ffi::imageio_source_copy_metadata_at_index(self.raw, index) })
    }

    /// Wraps `CGImageSourceCopyAuxiliaryDataInfoAtIndex`.
    pub fn auxiliary_data_at_index(
        &self,
        index: usize,
        auxiliary_type: AuxiliaryDataType,
    ) -> Result<Option<AuxiliaryDataInfo>, ImageError> {
        let auxiliary_type = bridge::cstring(auxiliary_type.identifier())?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_copy_auxiliary_data_at_index(
                self.raw,
                index,
                auxiliary_type.as_ptr(),
                buffer,
                size,
            )
        });
        if raw.is_null() && !message.is_empty() {
            return Err(ImageError::DecodeFailed(message));
        }
        Ok(AuxiliaryDataInfo::from_raw(raw))
    }

    /// Wraps `CGImageSourceCreateImageAtIndex`.
    pub fn decode_image_at_index(&self, index: usize) -> Result<DecodedImage, ImageError> {
        let mut width = 0_usize;
        let mut height = 0_usize;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_bgra_at_index(
                self.raw,
                index,
                &mut width,
                &mut height,
                buffer,
                size,
            )
        });
        if raw.is_null() {
            return Err(ImageError::DecodeFailed(if message.is_empty() {
                "imageio_source_create_bgra_at_index returned NULL".into()
            } else {
                message
            }));
        }
        Ok(DecodedImage {
            width,
            height,
            bgra: bridge::copy_data(raw),
        })
    }

    #[must_use]
    /// Wraps `CGImageSourceGetPrimaryImageIndex`.
    pub fn primary_image_index(&self) -> usize {
        unsafe { ffi::imageio_source_get_primary_image_index(self.raw) }
    }

    /// Wraps `CGImageSourceRemoveCacheAtIndex`.
    pub fn remove_cache_at_index(&self, index: usize) {
        unsafe { ffi::imageio_source_remove_cache_at_index(self.raw, index) };
    }
}

impl Clone for ImageSource {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}
