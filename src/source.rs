//! Safe wrapper around `CGImageSource`.

use std::path::Path;

use std::ffi::CString;

use crate::auxiliary_data::{AuxiliaryDataInfo, AuxiliaryDataType};
use crate::bridge::{self, source as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::limits::DecodeLimits;
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ImageSourceOptions {
    pub type_identifier_hint: Option<String>,
    pub should_cache: Option<bool>,
}

impl ImageSourceOptions {
    fn bridge_values(&self) -> Result<(Option<CString>, i8), ImageError> {
        let hint = self
            .type_identifier_hint
            .as_deref()
            .map(bridge::cstring)
            .transpose()?;
        Ok((hint, should_cache_flag(self.should_cache)))
    }
}

const fn should_cache_flag(should_cache: Option<bool>) -> i8 {
    match should_cache {
        None => -1,
        Some(false) => 0,
        Some(true) => 1,
    }
}

#[derive(Debug)]
pub struct DataProvider {
    raw: Handle,
}

impl DataProvider {
    pub fn from_bytes(data: &[u8]) -> Result<Self, ImageError> {
        let raw =
            unsafe { ffi::imageio_data_provider_create_with_bytes(data.as_ptr(), data.len()) };
        (!raw.is_null()).then_some(Self { raw }).ok_or_else(|| {
            ImageError::OpenSourceFailed("CGDataProviderCreateWithCFData returned NULL".into())
        })
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        let path = bridge::path_to_cstring(path.as_ref())?;
        let raw = unsafe { ffi::imageio_data_provider_create_with_path(path.as_ptr()) };
        (!raw.is_null()).then_some(Self { raw }).ok_or_else(|| {
            ImageError::OpenSourceFailed("CGDataProviderCreateWithURL returned NULL".into())
        })
    }
}

crate::bridge::retained::imageio_retained!(DataProvider);

/// Owned image source.
#[derive(Debug)]
pub struct ImageSource {
    raw: Handle,
    limits: DecodeLimits,
    should_cache: Option<bool>,
}

impl ImageSource {
    pub(crate) fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self {
            raw,
            limits: DecodeLimits::DEFAULT,
            should_cache: None,
        })
    }

    fn opened(
        raw: Handle,
        message: String,
        fallback: &str,
        options: &ImageSourceOptions,
    ) -> Result<Self, ImageError> {
        let mut source = Self::from_raw(raw).ok_or_else(|| {
            ImageError::OpenSourceFailed(if message.is_empty() {
                fallback.into()
            } else {
                message
            })
        })?;
        source.should_cache = options.should_cache;
        Ok(source)
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
        Self::from_path_with_options(path, &ImageSourceOptions::default())
    }

    pub fn from_path_with_options(
        path: impl AsRef<Path>,
        options: &ImageSourceOptions,
    ) -> Result<Self, ImageError> {
        let path = bridge::path_to_cstring(path.as_ref())?;
        let (hint, should_cache) = options.bridge_values()?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_from_path(
                path.as_ptr(),
                hint.as_ref().map_or(std::ptr::null(), |hint| hint.as_ptr()),
                should_cache,
                buffer,
                size,
            )
        });
        Self::opened(
            raw,
            message,
            "imageio_source_create_from_path returned NULL",
            options,
        )
    }

    /// Wraps `CGImageSourceCreateWithData`.
    pub fn from_bytes(data: &[u8]) -> Result<Self, ImageError> {
        Self::from_bytes_with_options(data, &ImageSourceOptions::default())
    }

    pub fn from_bytes_with_options(
        data: &[u8],
        options: &ImageSourceOptions,
    ) -> Result<Self, ImageError> {
        let (hint, should_cache) = options.bridge_values()?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_from_bytes(
                data.as_ptr(),
                data.len(),
                hint.as_ref().map_or(std::ptr::null(), |hint| hint.as_ptr()),
                should_cache,
                buffer,
                size,
            )
        });
        Self::opened(
            raw,
            message,
            "imageio_source_create_from_bytes returned NULL",
            options,
        )
    }

    pub fn from_data_provider(
        provider: &DataProvider,
        options: &ImageSourceOptions,
    ) -> Result<Self, ImageError> {
        let (hint, should_cache) = options.bridge_values()?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_with_data_provider(
                provider.raw,
                hint.as_ref().map_or(std::ptr::null(), |hint| hint.as_ptr()),
                should_cache,
                buffer,
                size,
            )
        });
        Self::opened(
            raw,
            message,
            "imageio_source_create_with_data_provider returned NULL",
            options,
        )
    }

    /// Wraps `CGImageSourceCreateIncremental`.
    pub fn incremental() -> Result<Self, ImageError> {
        Self::incremental_with_options(&ImageSourceOptions::default())
    }

    pub fn incremental_with_options(options: &ImageSourceOptions) -> Result<Self, ImageError> {
        let (hint, should_cache) = options.bridge_values()?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_incremental(
                hint.as_ref().map_or(std::ptr::null(), |hint| hint.as_ptr()),
                should_cache,
                buffer,
                size,
            )
        });
        Self::opened(
            raw,
            message,
            "imageio_source_create_incremental returned NULL",
            options,
        )
    }

    #[must_use]
    pub const fn decode_limits(&self) -> DecodeLimits {
        self.limits
    }

    pub fn set_decode_limits(&mut self, limits: DecodeLimits) {
        self.limits = limits;
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

    pub fn update_data_provider(
        &mut self,
        provider: &DataProvider,
        is_final: bool,
    ) -> Result<(), ImageError> {
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_update_data_provider(self.raw, provider.raw, is_final, buffer, size)
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::OpenSourceFailed(if message.is_empty() {
                "imageio_source_update_data_provider returned false".into()
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
        let (max_width, max_height, max_bytes) = self.limits.bridge_values();
        let mut width = 0_usize;
        let mut height = 0_usize;
        let mut limit_exceeded = false;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_source_create_bgra_at_index(
                self.raw,
                index,
                max_width,
                max_height,
                max_bytes,
                should_cache_flag(self.should_cache),
                &raw mut width,
                &raw mut height,
                &raw mut limit_exceeded,
                buffer,
                size,
            )
        });
        if limit_exceeded {
            return Err(ImageError::LimitExceeded {
                width,
                height,
                limits: self.limits,
            });
        }
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
            limits: self.limits,
            should_cache: self.should_cache,
        }
    }
}

crate::bridge::retained::imageio_retained!(ImageSource, drop_only);

#[cfg(test)]
mod tests {
    use super::{should_cache_flag, ImageSourceOptions, SourceStatus};
    use crate::error::ImageError;

    #[test]
    fn should_cache_maps_to_a_tri_state_flag() {
        assert_eq!(should_cache_flag(None), -1);
        assert_eq!(should_cache_flag(Some(false)), 0);
        assert_eq!(should_cache_flag(Some(true)), 1);
    }

    #[test]
    fn source_options_convert_the_type_hint() {
        let (hint, should_cache) = ImageSourceOptions::default()
            .bridge_values()
            .expect("default options");
        assert!(hint.is_none());
        assert_eq!(should_cache, -1);

        let options = ImageSourceOptions {
            type_identifier_hint: Some("public.heic".into()),
            should_cache: Some(true),
        };
        let (hint, should_cache) = options.bridge_values().expect("hinted options");
        assert_eq!(
            hint.as_deref().map(std::ffi::CStr::to_bytes),
            Some(&b"public.heic"[..])
        );
        assert_eq!(should_cache, 1);

        let invalid = ImageSourceOptions {
            type_identifier_hint: Some("public\0png".into()),
            should_cache: None,
        };
        assert!(matches!(
            invalid.bridge_values(),
            Err(ImageError::Unknown(_))
        ));
    }

    #[test]
    fn source_status_maps_known_numeric_values() {
        assert_eq!(SourceStatus::from(-5), SourceStatus::UnexpectedEof);
        assert_eq!(SourceStatus::from(-4), SourceStatus::InvalidData);
        assert_eq!(SourceStatus::from(-3), SourceStatus::UnknownType);
        assert_eq!(SourceStatus::from(-2), SourceStatus::ReadingHeader);
        assert_eq!(SourceStatus::from(-1), SourceStatus::Incomplete);
        assert_eq!(SourceStatus::from(0), SourceStatus::Complete);
    }

    #[test]
    fn source_status_preserves_unknown_numeric_values() {
        assert_eq!(SourceStatus::from(42), SourceStatus::Unknown(42));
        assert_eq!(SourceStatus::from(-42), SourceStatus::Unknown(-42));
    }

    #[test]
    fn source_status_progression_matches_core_graphics_ordering() {
        let statuses = [
            -5, -4, -3, -2, -1, 0,
        ];

        assert!(statuses.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
