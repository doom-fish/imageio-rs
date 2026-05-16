//! Safe wrappers around `CGImageMetadata` and `CGImageMetadataTag`.

use core::ptr;
use std::cell::RefCell;

use block2::RcBlock;

use crate::block_support::CBool;
use crate::error::ImageError;
use crate::ffi;
use crate::util::{
    cf_data_to_vec, cf_error_description, cf_string_to_string, cf_type_to_string, make_cf_data,
    make_cf_string,
};

/// Immutable `ImageIO` metadata tree.
#[derive(Debug)]
pub struct Metadata {
    raw: ffi::CGImageMetadataRef,
}

impl Metadata {
    pub(crate) fn from_raw(raw: ffi::CGImageMetadataRef) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    /// Parse `ImageIO` XMP data into a metadata tree.
    pub fn from_xmp_data(data: &[u8]) -> Result<Self, ImageError> {
        let cf_data = make_cf_data(data)?;
        let metadata = unsafe { ffi::CGImageMetadataCreateFromXMPData(cf_data) };
        unsafe { ffi::CFRelease(cf_data.cast()) };
        Self::from_raw(metadata).ok_or_else(|| {
            ImageError::DecodeFailed("CGImageMetadataCreateFromXMPData returned NULL".into())
        })
    }

    /// Serialize the metadata tree to XMP bytes.
    pub fn create_xmp_data(&self) -> Result<Vec<u8>, ImageError> {
        let data = unsafe { ffi::CGImageMetadataCreateXMPData(self.raw, ptr::null()) };
        if data.is_null() {
            return Err(ImageError::EncodeFailed(
                "CGImageMetadataCreateXMPData returned NULL".into(),
            ));
        }
        let bytes = cf_data_to_vec(data);
        unsafe { ffi::CFRelease(data.cast()) };
        Ok(bytes)
    }

    /// Copy all top-level tags in the metadata tree.
    #[must_use]
    pub fn tags(&self) -> Vec<MetadataTag> {
        let array = unsafe { ffi::CGImageMetadataCopyTags(self.raw) };
        if array.is_null() {
            return Vec::new();
        }
        let count = usize::try_from(unsafe { ffi::CFArrayGetCount(array) }).unwrap_or(0);
        let mut tags = Vec::with_capacity(count);
        for index in 0..count {
            let tag = unsafe {
                ffi::CFArrayGetValueAtIndex(array, ffi::CFIndex::try_from(index).unwrap_or(0))
                    .cast()
            };
            if let Some(tag) = unsafe { MetadataTag::retain_from_borrowed(tag) } {
                tags.push(tag);
            }
        }
        unsafe { ffi::CFRelease(array.cast()) };
        tags
    }

    /// Copy a tag by `ImageIO` path.
    pub fn tag_with_path(&self, path: &str) -> Result<Option<MetadataTag>, ImageError> {
        let path = make_cf_string(path)?;
        let tag = unsafe { ffi::CGImageMetadataCopyTagWithPath(self.raw, ptr::null(), path) };
        unsafe { ffi::CFRelease(path.cast()) };
        Ok(MetadataTag::from_raw(tag))
    }

    /// Copy a string value by `ImageIO` path.
    pub fn string_value_with_path(&self, path: &str) -> Result<Option<String>, ImageError> {
        let path = make_cf_string(path)?;
        let value =
            unsafe { ffi::CGImageMetadataCopyStringValueWithPath(self.raw, ptr::null(), path) };
        unsafe { ffi::CFRelease(path.cast()) };
        let string = cf_string_to_string(value);
        if !value.is_null() {
            unsafe { ffi::CFRelease(value.cast()) };
        }
        Ok(string)
    }

    /// Enumerate tags under `root_path`.
    pub fn enumerate_tags(
        &self,
        root_path: Option<&str>,
        callback: impl FnMut(String, MetadataTag) -> bool + 'static,
    ) -> Result<(), ImageError> {
        let root_path = root_path.map(make_cf_string).transpose()?;
        let callback = RefCell::new(callback);
        let block: RcBlock<dyn Fn(ffi::CFStringRef, ffi::CGImageMetadataTagRef) -> CBool> =
            RcBlock::new(
                move |path: ffi::CFStringRef, tag: ffi::CGImageMetadataTagRef| -> CBool {
                    let path = cf_string_to_string(path).unwrap_or_default();
                    let Some(tag) = (unsafe { MetadataTag::retain_from_borrowed(tag) }) else {
                        return CBool::from(false);
                    };
                    CBool::from(callback.borrow_mut()(path, tag))
                },
            );
        let block_ptr: *const block2::Block<
            dyn Fn(ffi::CFStringRef, ffi::CGImageMetadataTagRef) -> CBool,
        > = &*block;
        unsafe {
            ffi::CGImageMetadataEnumerateTagsUsingBlock(
                self.raw,
                root_path.as_ref().copied().unwrap_or(ptr::null()),
                ptr::null(),
                block_ptr.cast(),
            );
        }
        if let Some(root_path) = root_path {
            unsafe { ffi::CFRelease(root_path.cast()) };
        }
        Ok(())
    }

    /// Copy the metadata tag that corresponds to an `ImageIO` property key.
    ///
    /// # Safety
    ///
    /// `dictionary_name` and `property_name` must be valid `CFStringRef` values,
    /// such as the exported `kCGImageProperty*` constants from [`crate::ffi`].
    #[must_use]
    pub unsafe fn tag_matching_image_property(
        &self,
        dictionary_name: ffi::CFStringRef,
        property_name: ffi::CFStringRef,
    ) -> Option<MetadataTag> {
        let tag = unsafe {
            ffi::CGImageMetadataCopyTagMatchingImageProperty(
                self.raw,
                dictionary_name,
                property_name,
            )
        };
        MetadataTag::from_raw(tag)
    }

    #[must_use]
    pub const fn as_raw(&self) -> ffi::CGImageMetadataRef {
        self.raw
    }
}

impl Clone for Metadata {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::CFRetain(self.raw.cast()).cast() };
        Self { raw }
    }
}

impl Drop for Metadata {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.raw.cast()) };
    }
}

/// Mutable `ImageIO` metadata tree.
#[derive(Debug)]
pub struct MutableMetadata {
    raw: ffi::CGMutableImageMetadataRef,
}

impl MutableMetadata {
    /// Create an empty mutable metadata tree.
    pub fn new() -> Result<Self, ImageError> {
        let raw = unsafe { ffi::CGImageMetadataCreateMutable() };
        (!raw.is_null())
            .then_some(Self { raw })
            .ok_or_else(|| ImageError::Unknown("CGImageMetadataCreateMutable returned NULL".into()))
    }

    /// Create a mutable copy of an existing metadata tree.
    pub fn copy_from(metadata: &Metadata) -> Result<Self, ImageError> {
        let raw = unsafe { ffi::CGImageMetadataCreateMutableCopy(metadata.raw) };
        (!raw.is_null()).then_some(Self { raw }).ok_or_else(|| {
            ImageError::Unknown("CGImageMetadataCreateMutableCopy returned NULL".into())
        })
    }

    /// Convert into an immutable metadata wrapper without changing ownership.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_metadata(self) -> Metadata {
        let raw = self.raw.cast();
        std::mem::forget(self);
        Metadata { raw }
    }

    /// Register a namespace/prefix mapping.
    pub fn register_namespace_for_prefix(
        &mut self,
        xmlns: &str,
        prefix: &str,
    ) -> Result<(), ImageError> {
        let xmlns = make_cf_string(xmlns)?;
        let prefix = make_cf_string(prefix)?;
        let mut error: ffi::CFErrorRef = ptr::null();
        let ok = unsafe {
            ffi::CGImageMetadataRegisterNamespaceForPrefix(
                self.raw,
                xmlns,
                prefix,
                ptr::from_mut(&mut error),
            )
        };
        unsafe {
            ffi::CFRelease(xmlns.cast());
            ffi::CFRelease(prefix.cast());
        }
        if ok {
            return Ok(());
        }
        let message = cf_error_description(error)
            .unwrap_or_else(|| "CGImageMetadataRegisterNamespaceForPrefix returned false".into());
        if !error.is_null() {
            unsafe { ffi::CFRelease(error.cast()) };
        }
        Err(ImageError::Unknown(message))
    }

    /// Set a string tag at `path`.
    pub fn set_tag_with_path(&mut self, path: &str, tag: &MetadataTag) -> Result<(), ImageError> {
        let path = make_cf_string(path)?;
        let ok =
            unsafe { ffi::CGImageMetadataSetTagWithPath(self.raw, ptr::null(), path, tag.raw) };
        unsafe { ffi::CFRelease(path.cast()) };
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(
                "CGImageMetadataSetTagWithPath returned false".into(),
            ))
        }
    }

    /// Set a plain string value at `path`.
    pub fn set_string_value_with_path(
        &mut self,
        path: &str,
        value: &str,
    ) -> Result<(), ImageError> {
        let path = make_cf_string(path)?;
        let value = make_cf_string(value)?;
        let ok = unsafe {
            ffi::CGImageMetadataSetValueWithPath(self.raw, ptr::null(), path, value.cast())
        };
        unsafe {
            ffi::CFRelease(path.cast());
            ffi::CFRelease(value.cast());
        }
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(
                "CGImageMetadataSetValueWithPath returned false".into(),
            ))
        }
    }

    /// Remove a tag at `path`.
    pub fn remove_tag_with_path(&mut self, path: &str) -> Result<(), ImageError> {
        let path = make_cf_string(path)?;
        let ok = unsafe { ffi::CGImageMetadataRemoveTagWithPath(self.raw, ptr::null(), path) };
        unsafe { ffi::CFRelease(path.cast()) };
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(
                "CGImageMetadataRemoveTagWithPath returned false".into(),
            ))
        }
    }

    /// Set a string value matching an `ImageIO` property key.
    ///
    /// # Safety
    ///
    /// `dictionary_name` and `property_name` must be valid `CFStringRef` values,
    /// such as the exported `kCGImageProperty*` constants from [`crate::ffi`].
    pub unsafe fn set_string_value_matching_image_property(
        &mut self,
        dictionary_name: ffi::CFStringRef,
        property_name: ffi::CFStringRef,
        value: &str,
    ) -> Result<(), ImageError> {
        let value = make_cf_string(value)?;
        let ok = unsafe {
            ffi::CGImageMetadataSetValueMatchingImageProperty(
                self.raw,
                dictionary_name,
                property_name,
                value.cast(),
            )
        };
        unsafe { ffi::CFRelease(value.cast()) };
        if ok {
            Ok(())
        } else {
            Err(ImageError::Unknown(
                "CGImageMetadataSetValueMatchingImageProperty returned false".into(),
            ))
        }
    }

    #[must_use]
    pub const fn as_raw(&self) -> ffi::CGMutableImageMetadataRef {
        self.raw
    }
}

impl Clone for MutableMetadata {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::CFRetain(self.raw.cast()).cast_mut() };
        Self { raw }
    }
}

impl Drop for MutableMetadata {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.raw.cast()) };
    }
}

/// A single metadata tag.
#[derive(Debug)]
pub struct MetadataTag {
    raw: ffi::CGImageMetadataTagRef,
}

impl MetadataTag {
    pub(crate) fn from_raw(raw: ffi::CGImageMetadataTagRef) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    pub(crate) unsafe fn retain_from_borrowed(raw: ffi::CGImageMetadataTagRef) -> Option<Self> {
        if raw.is_null() {
            return None;
        }
        let retained = unsafe { ffi::CFRetain(raw.cast()).cast() };
        Some(Self { raw: retained })
    }

    /// Create a string metadata tag.
    pub fn new_string(
        xmlns: &str,
        prefix: Option<&str>,
        name: &str,
        value: &str,
    ) -> Result<Self, ImageError> {
        let xmlns = make_cf_string(xmlns)?;
        let prefix = prefix.map(make_cf_string).transpose()?;
        let name = make_cf_string(name)?;
        let value = make_cf_string(value)?;
        let tag = unsafe {
            ffi::CGImageMetadataTagCreate(
                xmlns,
                prefix.as_ref().copied().unwrap_or(ptr::null()),
                name,
                ffi::kCGImageMetadataTypeString,
                value.cast(),
            )
        };
        unsafe {
            ffi::CFRelease(xmlns.cast());
            ffi::CFRelease(name.cast());
            ffi::CFRelease(value.cast());
        }
        if let Some(prefix) = prefix {
            unsafe { ffi::CFRelease(prefix.cast()) };
        }
        Self::from_raw(tag)
            .ok_or_else(|| ImageError::Unknown("CGImageMetadataTagCreate returned NULL".into()))
    }

    fn copy_string(
        &self,
        getter: unsafe extern "C" fn(ffi::CGImageMetadataTagRef) -> ffi::CFStringRef,
    ) -> Option<String> {
        let value = unsafe { getter(self.raw) };
        let string = cf_string_to_string(value);
        if !value.is_null() {
            unsafe { ffi::CFRelease(value.cast()) };
        }
        string
    }

    /// Tag namespace URI.
    #[must_use]
    pub fn namespace(&self) -> Option<String> {
        self.copy_string(ffi::CGImageMetadataTagCopyNamespace)
    }

    /// Tag namespace prefix.
    #[must_use]
    pub fn prefix(&self) -> Option<String> {
        self.copy_string(ffi::CGImageMetadataTagCopyPrefix)
    }

    /// Tag name.
    #[must_use]
    pub fn name(&self) -> Option<String> {
        self.copy_string(ffi::CGImageMetadataTagCopyName)
    }

    /// Tag value when it is a string.
    #[must_use]
    pub fn string_value(&self) -> Option<String> {
        let value = unsafe { ffi::CGImageMetadataTagCopyValue(self.raw) };
        let string = cf_type_to_string(value);
        if !value.is_null() {
            unsafe { ffi::CFRelease(value.cast()) };
        }
        string
    }

    /// `ImageIO` tag type.
    #[must_use]
    pub fn tag_type(&self) -> ffi::CGImageMetadataType {
        unsafe { ffi::CGImageMetadataTagGetType(self.raw) }
    }

    /// Tag qualifiers.
    #[must_use]
    pub fn qualifiers(&self) -> Vec<Self> {
        let array = unsafe { ffi::CGImageMetadataTagCopyQualifiers(self.raw) };
        if array.is_null() {
            return Vec::new();
        }
        let count = usize::try_from(unsafe { ffi::CFArrayGetCount(array) }).unwrap_or(0);
        let mut qualifiers = Vec::with_capacity(count);
        for index in 0..count {
            let tag = unsafe {
                ffi::CFArrayGetValueAtIndex(array, ffi::CFIndex::try_from(index).unwrap_or(0))
                    .cast()
            };
            if let Some(tag) = unsafe { Self::retain_from_borrowed(tag) } {
                qualifiers.push(tag);
            }
        }
        unsafe { ffi::CFRelease(array.cast()) };
        qualifiers
    }

    #[must_use]
    pub const fn as_raw(&self) -> ffi::CGImageMetadataTagRef {
        self.raw
    }
}

impl Clone for MetadataTag {
    fn clone(&self) -> Self {
        let raw = unsafe { ffi::CFRetain(self.raw.cast()).cast() };
        Self { raw }
    }
}

impl Drop for MetadataTag {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.raw.cast()) };
    }
}
