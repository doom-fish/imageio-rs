//! Generic `ImageIO` property dictionaries.

use crate::bridge::{self, properties as ffi, Handle};
use crate::error::ImageError;

/// Immutable `ImageIO` property dictionary.
#[derive(Debug)]
pub struct ImageProperties {
    raw: Handle,
}

impl ImageProperties {
    pub(crate) fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> Handle {
        self.raw
    }

    #[must_use]
    /// Returns the dictionary keys available in this `ImageIO` property map.
    pub fn keys(&self) -> Vec<String> {
        bridge::copy_string_array(unsafe { ffi::imageio_properties_copy_keys(self.raw) })
    }

    /// Checks whether a property key is present.
    pub fn has_key(&self, key: &str) -> Result<bool, ImageError> {
        let key = bridge::cstring(key)?;
        Ok(unsafe { ffi::imageio_properties_has_key(self.raw, key.as_ptr()) })
    }

    /// Reads a string property.
    pub fn string(&self, key: &str) -> Result<Option<String>, ImageError> {
        let key = bridge::cstring(key)?;
        Ok(bridge::copy_string(unsafe {
            ffi::imageio_properties_copy_string(self.raw, key.as_ptr())
        }))
    }

    /// Reads an integer property backed by `CFNumberRef`.
    pub fn i64(&self, key: &str) -> Result<Option<i64>, ImageError> {
        let key = bridge::cstring(key)?;
        let mut value = 0_i64;
        let found = unsafe { ffi::imageio_properties_get_i64(self.raw, key.as_ptr(), &mut value) };
        Ok(found.then_some(value))
    }

    /// Reads a floating-point property backed by `CFNumberRef`.
    pub fn f64(&self, key: &str) -> Result<Option<f64>, ImageError> {
        let key = bridge::cstring(key)?;
        let mut value = 0.0_f64;
        let found = unsafe { ffi::imageio_properties_get_f64(self.raw, key.as_ptr(), &mut value) };
        Ok(found.then_some(value))
    }

    /// Reads a Boolean property backed by `CFBooleanRef`.
    pub fn bool(&self, key: &str) -> Result<Option<bool>, ImageError> {
        let key = bridge::cstring(key)?;
        let mut value = false;
        let found = unsafe { ffi::imageio_properties_get_bool(self.raw, key.as_ptr(), &mut value) };
        Ok(found.then_some(value))
    }

    /// Reads a nested `ImageIO` property dictionary.
    pub fn dictionary(&self, key: &str) -> Result<Option<Self>, ImageError> {
        let key = bridge::cstring(key)?;
        Ok(Self::from_raw(unsafe {
            ffi::imageio_properties_copy_dictionary(self.raw, key.as_ptr())
        }))
    }
}

impl Clone for ImageProperties {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for ImageProperties {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}

/// Mutable property dictionary builder.
#[derive(Debug)]
pub struct MutableProperties {
    raw: Handle,
}

impl MutableProperties {
    /// Creates a mutable property dictionary for `CGImageDestinationSetProperties`.
    pub fn new() -> Result<Self, ImageError> {
        let raw = unsafe { ffi::imageio_mutable_properties_create() };
        (!raw.is_null()).then_some(Self { raw }).ok_or_else(|| {
            ImageError::Unknown("imageio_mutable_properties_create returned NULL".into())
        })
    }

    /// Sets a string property.
    pub fn set_string(&mut self, key: &str, value: &str) -> Result<(), ImageError> {
        let key = bridge::cstring(key)?;
        let value = bridge::cstring(value)?;
        unsafe {
            ffi::imageio_mutable_properties_set_string(self.raw, key.as_ptr(), value.as_ptr());
        }
        Ok(())
    }

    /// Sets an integer property.
    pub fn set_i64(&mut self, key: &str, value: i64) -> Result<(), ImageError> {
        let key = bridge::cstring(key)?;
        unsafe { ffi::imageio_mutable_properties_set_i64(self.raw, key.as_ptr(), value) };
        Ok(())
    }

    /// Sets a floating-point property.
    pub fn set_f64(&mut self, key: &str, value: f64) -> Result<(), ImageError> {
        let key = bridge::cstring(key)?;
        unsafe { ffi::imageio_mutable_properties_set_f64(self.raw, key.as_ptr(), value) };
        Ok(())
    }

    /// Sets a Boolean property.
    pub fn set_bool(&mut self, key: &str, value: bool) -> Result<(), ImageError> {
        let key = bridge::cstring(key)?;
        unsafe { ffi::imageio_mutable_properties_set_bool(self.raw, key.as_ptr(), value) };
        Ok(())
    }

    /// Sets a nested `ImageIO` property dictionary.
    pub fn set_dictionary(&mut self, key: &str, value: &ImageProperties) -> Result<(), ImageError> {
        let key = bridge::cstring(key)?;
        unsafe {
            ffi::imageio_mutable_properties_set_dictionary(self.raw, key.as_ptr(), value.as_raw());
        }
        Ok(())
    }

    /// Freezes this builder into an immutable property dictionary.
    pub fn freeze(&self) -> Result<ImageProperties, ImageError> {
        ImageProperties::from_raw(unsafe { ffi::imageio_mutable_properties_freeze(self.raw) })
            .ok_or_else(|| {
                ImageError::Unknown("imageio_mutable_properties_freeze returned NULL".into())
            })
    }
}

impl Clone for MutableProperties {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for MutableProperties {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}
