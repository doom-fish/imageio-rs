//! Metadata and metadata-tag wrappers.

use std::ffi::c_void;

use doom_fish_utils::panic_safe::catch_user_panic;

use crate::bridge::{self, metadata as ffi, Handle};
use crate::error::ImageError;

/// Metadata tag types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MetadataType {
    /// Matches `kCGImageMetadataTypeInvalid`.
    Invalid,
    /// Matches `kCGImageMetadataTypeDefault`.
    Default,
    /// Matches `kCGImageMetadataTypeString`.
    String,
    /// Matches `kCGImageMetadataTypeArrayUnordered`.
    ArrayUnordered,
    /// Matches `kCGImageMetadataTypeArrayOrdered`.
    ArrayOrdered,
    /// Matches `kCGImageMetadataTypeAlternateArray`.
    AlternateArray,
    /// Matches `kCGImageMetadataTypeAlternateText`.
    AlternateText,
    /// Matches `kCGImageMetadataTypeStructure`.
    Structure,
    /// Preserves an unknown `CGImageMetadataType` discriminator.
    Unknown(i32),
}

impl From<i32> for MetadataType {
    fn from(value: i32) -> Self {
        match value {
            -1 => Self::Invalid,
            0 => Self::Default,
            1 => Self::String,
            2 => Self::ArrayUnordered,
            3 => Self::ArrayOrdered,
            4 => Self::AlternateArray,
            5 => Self::AlternateText,
            6 => Self::Structure,
            other => Self::Unknown(other),
        }
    }
}

/// Options for [`Metadata::enumerate_tags_with_options`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetadataEnumerateOptions {
    /// Enables the `kCGImageMetadataEnumerateRecursively` option.
    pub recursive: bool,
}

impl MetadataEnumerateOptions {
    #[must_use]
    /// Creates non-recursive metadata-enumeration options.
    pub const fn new() -> Self {
        Self { recursive: false }
    }

    #[must_use]
    /// Creates recursive metadata-enumeration options.
    pub const fn recursive() -> Self {
        Self { recursive: true }
    }

    #[must_use]
    /// Sets whether `CGImageMetadataEnumerateTagsUsingBlock` should recurse.
    pub const fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
}

impl Default for MetadataEnumerateOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable metadata tree.
#[derive(Debug)]
pub struct Metadata {
    raw: Handle,
}

impl Metadata {
    pub(crate) fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> Handle {
        self.raw
    }

    #[must_use]
    /// Returns `kCFErrorDomainCGImageMetadata`.
    pub fn error_domain() -> String {
        bridge::copy_string(unsafe { ffi::imageio_metadata_error_domain() })
            .unwrap_or_else(|| "kCFErrorDomainCGImageMetadata".into())
    }

    /// Wraps `CGImageMetadataCreateFromXMPData`.
    pub fn from_xmp_data(data: &[u8]) -> Result<Self, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_create_from_xmp_data(data.as_ptr(), data.len(), buffer, size)
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::DecodeFailed(if message.is_empty() {
                "imageio_metadata_create_from_xmp_data returned NULL".into()
            } else {
                message
            })
        })
    }

    /// Wraps `CGImageMetadataCreateXMPData`.
    pub fn create_xmp_data(&self) -> Result<Vec<u8>, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_create_xmp_data(self.raw, buffer, size)
        });
        if raw.is_null() {
            return Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_metadata_create_xmp_data returned NULL".into()
            } else {
                message
            }));
        }
        Ok(bridge::copy_data(raw))
    }

    #[must_use]
    /// Wraps `CGImageMetadataCopyTags`.
    pub fn tags(&self) -> Vec<MetadataTag> {
        let array = unsafe { ffi::imageio_metadata_copy_tags(self.raw) };
        if array.is_null() {
            return Vec::new();
        }
        let count = unsafe { ffi::imageio_metadata_tag_array_count(array) };
        let mut tags = Vec::with_capacity(count);
        for index in 0..count {
            if let Some(tag) = MetadataTag::from_raw(unsafe {
                ffi::imageio_metadata_tag_array_copy_item(array, index)
            }) {
                tags.push(tag);
            }
        }
        bridge::release(array);
        tags
    }

    /// Wraps `CGImageMetadataCopyTagWithPath`.
    pub fn tag_with_path(&self, path: &str) -> Result<Option<MetadataTag>, ImageError> {
        let path = bridge::cstring(path)?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_copy_tag_with_path(self.raw, path.as_ptr(), buffer, size)
        });
        if raw.is_null() && !message.is_empty() {
            return Err(ImageError::DecodeFailed(message));
        }
        Ok(MetadataTag::from_raw(raw))
    }

    /// Wraps `CGImageMetadataCopyStringValueWithPath`.
    pub fn string_value_with_path(&self, path: &str) -> Result<Option<String>, ImageError> {
        let path = bridge::cstring(path)?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_copy_string_value_with_path(self.raw, path.as_ptr(), buffer, size)
        });
        if raw.is_null() && !message.is_empty() {
            return Err(ImageError::DecodeFailed(message));
        }
        Ok(bridge::copy_string(raw))
    }

    /// Wraps `CGImageMetadataEnumerateTagsUsingBlock` with default options.
    pub fn enumerate_tags<F>(&self, root_path: Option<&str>, callback: F) -> Result<(), ImageError>
    where
        F: FnMut(String, MetadataTag) -> bool,
    {
        self.enumerate_tags_with_options(root_path, MetadataEnumerateOptions::default(), callback)
    }

    /// Wraps `CGImageMetadataEnumerateTagsUsingBlock` with explicit options.
    pub fn enumerate_tags_with_options<F>(
        &self,
        root_path: Option<&str>,
        options: MetadataEnumerateOptions,
        callback: F,
    ) -> Result<(), ImageError>
    where
        F: FnMut(String, MetadataTag) -> bool,
    {
        struct EnumerationState<F> {
            callback: F,
        }

        unsafe extern "C" fn trampoline<F>(
            path: Handle,
            tag: Handle,
            user_data: *mut c_void,
        ) -> bool
        where
            F: FnMut(String, MetadataTag) -> bool,
        {
            let state = unsafe { &mut *user_data.cast::<EnumerationState<F>>() };
            let mut result = false;
            catch_user_panic("metadata_enumerate_trampoline", || {
                let path = bridge::copy_string(path).unwrap_or_default();
                let Some(tag) = MetadataTag::from_raw(tag) else {
                    return;
                };
                result = (state.callback)(path, tag);
            });
            result
        }

        let root_path = root_path.map(bridge::cstring).transpose()?;
        let mut state = EnumerationState { callback };
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_enumerate_tags(
                self.raw,
                root_path
                    .as_ref()
                    .map_or(std::ptr::null(), |path| path.as_ptr()),
                options.recursive,
                std::ptr::addr_of_mut!(state).cast::<c_void>(),
                trampoline::<F>,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::DecodeFailed(if message.is_empty() {
                "imageio_metadata_enumerate_tags returned false".into()
            } else {
                message
            }))
        }
    }
}

impl Clone for Metadata {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for Metadata {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}

/// Mutable metadata tree.
#[derive(Debug)]
pub struct MutableMetadata {
    raw: Handle,
}

impl MutableMetadata {
    /// Wraps `CGImageMetadataCreateMutable`.
    pub fn new() -> Result<Self, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_mutable_metadata_create(buffer, size)
        });
        (!raw.is_null()).then_some(Self { raw }).ok_or_else(|| {
            ImageError::Unknown(if message.is_empty() {
                "imageio_mutable_metadata_create returned NULL".into()
            } else {
                message
            })
        })
    }

    /// Creates a mutable copy with `CGImageMetadataCreateMutableCopy`.
    pub fn copy_from(metadata: &Metadata) -> Result<Self, ImageError> {
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_mutable_metadata_create_copy(metadata.as_raw(), buffer, size)
        });
        (!raw.is_null()).then_some(Self { raw }).ok_or_else(|| {
            ImageError::Unknown(if message.is_empty() {
                "imageio_mutable_metadata_create_copy returned NULL".into()
            } else {
                message
            })
        })
    }

    #[must_use]
    /// Converts this mutable tree into an immutable `CGImageMetadataRef`.
    pub fn into_metadata(self) -> Metadata {
        let raw = unsafe { ffi::imageio_mutable_metadata_into_immutable(self.raw) };
        Metadata::from_raw(raw).unwrap_or_else(|| Metadata { raw: self.raw })
    }

    /// Wraps `CGImageMetadataRegisterNamespaceForPrefix`.
    pub fn register_namespace_for_prefix(
        &mut self,
        xmlns: &str,
        prefix: &str,
    ) -> Result<(), ImageError> {
        let xmlns = bridge::cstring(xmlns)?;
        let prefix = bridge::cstring(prefix)?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_register_namespace_for_prefix(
                self.raw,
                xmlns.as_ptr(),
                prefix.as_ptr(),
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(if message.is_empty() {
                "imageio_metadata_register_namespace_for_prefix returned false".into()
            } else {
                message
            }))
        }
    }

    /// Wraps `CGImageMetadataSetTagWithPath`.
    pub fn set_tag_with_path(&mut self, path: &str, tag: &MetadataTag) -> Result<(), ImageError> {
        let path = bridge::cstring(path)?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_set_tag_with_path(
                self.raw,
                path.as_ptr(),
                tag.as_raw(),
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(if message.is_empty() {
                "imageio_metadata_set_tag_with_path returned false".into()
            } else {
                message
            }))
        }
    }

    /// Wraps `CGImageMetadataSetValueWithPath` for string payloads.
    pub fn set_string_value_with_path(
        &mut self,
        path: &str,
        value: &str,
    ) -> Result<(), ImageError> {
        let path = bridge::cstring(path)?;
        let value = bridge::cstring(value)?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_set_string_value_with_path(
                self.raw,
                path.as_ptr(),
                value.as_ptr(),
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(if message.is_empty() {
                "imageio_metadata_set_string_value_with_path returned false".into()
            } else {
                message
            }))
        }
    }

    /// Wraps `CGImageMetadataRemoveTagWithPath`.
    pub fn remove_tag_with_path(&mut self, path: &str) -> Result<(), ImageError> {
        let path = bridge::cstring(path)?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_remove_tag_with_path(self.raw, path.as_ptr(), buffer, size)
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(if message.is_empty() {
                "imageio_metadata_remove_tag_with_path returned false".into()
            } else {
                message
            }))
        }
    }
}

impl Clone for MutableMetadata {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for MutableMetadata {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}

/// Owned metadata tag.
#[derive(Debug)]
pub struct MetadataTag {
    raw: Handle,
}

impl MetadataTag {
    pub(crate) fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> Handle {
        self.raw
    }

    /// Wraps `CGImageMetadataTagCreate` for string-valued tags.
    pub fn new_string(
        xmlns: &str,
        prefix: Option<&str>,
        name: &str,
        value: &str,
    ) -> Result<Self, ImageError> {
        let xmlns = bridge::cstring(xmlns)?;
        let prefix = prefix.map(bridge::cstring).transpose()?;
        let name = bridge::cstring(name)?;
        let value = bridge::cstring(value)?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_metadata_tag_create_string(
                xmlns.as_ptr(),
                prefix
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                name.as_ptr(),
                value.as_ptr(),
                buffer,
                size,
            )
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::Unknown(if message.is_empty() {
                "imageio_metadata_tag_create_string returned NULL".into()
            } else {
                message
            })
        })
    }

    #[must_use]
    /// Wraps `CGImageMetadataTagCopyNamespace`.
    pub fn namespace(&self) -> Option<String> {
        bridge::copy_string(unsafe { ffi::imageio_metadata_tag_copy_namespace(self.raw) })
    }

    #[must_use]
    /// Wraps `CGImageMetadataTagCopyPrefix`.
    pub fn prefix(&self) -> Option<String> {
        bridge::copy_string(unsafe { ffi::imageio_metadata_tag_copy_prefix(self.raw) })
    }

    #[must_use]
    /// Wraps `CGImageMetadataTagCopyName`.
    pub fn name(&self) -> Option<String> {
        bridge::copy_string(unsafe { ffi::imageio_metadata_tag_copy_name(self.raw) })
    }

    #[must_use]
    /// Wraps `CGImageMetadataTagCopyValue` for string payloads.
    pub fn string_value(&self) -> Option<String> {
        bridge::copy_string(unsafe { ffi::imageio_metadata_tag_copy_string_value(self.raw) })
    }

    #[must_use]
    /// Wraps `CGImageMetadataTagGetType`.
    pub fn tag_type(&self) -> MetadataType {
        unsafe { ffi::imageio_metadata_tag_get_type(self.raw) }.into()
    }

    #[must_use]
    /// Wraps `CGImageMetadataTagCopyQualifiers`.
    pub fn qualifiers(&self) -> Vec<Self> {
        let array = unsafe { ffi::imageio_metadata_tag_copy_qualifiers(self.raw) };
        if array.is_null() {
            return Vec::new();
        }
        let count = unsafe { ffi::imageio_metadata_tag_array_count(array) };
        let mut qualifiers = Vec::with_capacity(count);
        for index in 0..count {
            if let Some(tag) =
                Self::from_raw(unsafe { ffi::imageio_metadata_tag_array_copy_item(array, index) })
            {
                qualifiers.push(tag);
            }
        }
        bridge::release(array);
        qualifiers
    }
}

impl Clone for MetadataTag {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for MetadataTag {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}

#[cfg(test)]
mod tests {
    use super::{MetadataEnumerateOptions, MetadataType};
    use crate::ffi;

    #[test]
    fn metadata_type_maps_known_numeric_values() {
        assert_eq!(MetadataType::from(ffi::kCGImageMetadataTypeInvalid), MetadataType::Invalid);
        assert_eq!(MetadataType::from(ffi::kCGImageMetadataTypeDefault), MetadataType::Default);
        assert_eq!(MetadataType::from(ffi::kCGImageMetadataTypeString), MetadataType::String);
        assert_eq!(
            MetadataType::from(ffi::kCGImageMetadataTypeArrayUnordered),
            MetadataType::ArrayUnordered
        );
        assert_eq!(
            MetadataType::from(ffi::kCGImageMetadataTypeArrayOrdered),
            MetadataType::ArrayOrdered
        );
        assert_eq!(
            MetadataType::from(ffi::kCGImageMetadataTypeAlternateArray),
            MetadataType::AlternateArray
        );
        assert_eq!(
            MetadataType::from(ffi::kCGImageMetadataTypeAlternateText),
            MetadataType::AlternateText
        );
        assert_eq!(MetadataType::from(ffi::kCGImageMetadataTypeStructure), MetadataType::Structure);
    }

    #[test]
    fn metadata_type_preserves_unknown_numeric_values() {
        assert_eq!(MetadataType::from(77), MetadataType::Unknown(77));
        assert_eq!(MetadataType::from(-99), MetadataType::Unknown(-99));
    }

    #[test]
    fn metadata_enumerate_options_default_is_non_recursive() {
        let options = MetadataEnumerateOptions::default();

        assert_eq!(options, MetadataEnumerateOptions::new());
        assert!(!options.recursive);
    }

    #[test]
    fn metadata_enumerate_options_builder_toggles_recursive_flag() {
        assert_eq!(
            MetadataEnumerateOptions::recursive(),
            MetadataEnumerateOptions { recursive: true }
        );
        assert_eq!(
            MetadataEnumerateOptions::new().with_recursive(true),
            MetadataEnumerateOptions::recursive()
        );
        assert_eq!(
            MetadataEnumerateOptions::recursive().with_recursive(false),
            MetadataEnumerateOptions::new()
        );
    }
}
