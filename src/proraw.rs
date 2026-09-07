//! RAW, DNG, and Apple `ProRAW` property helpers.

use crate::bridge::{self, proraw as ffi};
use crate::destination::ImageDestination;
use crate::error::ImageError;
use crate::properties::{ImageProperties, MutableProperties};
use crate::source::ImageSource;

/// Maps to the `kCGImagePropertyRawDictionary` property dictionary.
pub const RAW_DICTIONARY_KEY: &str = "{Raw}";
/// Maps to the `kCGImagePropertyDNGDictionary` property dictionary.
pub const DNG_DICTIONARY_KEY: &str = "{DNG}";
/// Maps to the `kCGImagePropertyDNGProfileName` entry.
pub const PROFILE_NAME_KEY: &str = "DNGProfileName";
/// Maps to the `kCGImagePropertyDNGUniqueCameraModel` entry.
pub const UNIQUE_CAMERA_MODEL_KEY: &str = "UniqueCameraModel";
/// Canonical DNG uniform type identifier.
pub const DNG_TYPE_IDENTIFIER: &str = "com.adobe.raw-image";

/// Typed RAW/DNG property view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProRawProperties {
    /// Whether `kCGImagePropertyRawDictionary` is present.
    pub has_raw_dictionary: bool,
    /// Whether `kCGImagePropertyDNGDictionary` is present.
    pub has_dng_dictionary: bool,
    /// Decoded value of `kCGImagePropertyDNGProfileName`.
    pub profile_name: Option<String>,
    /// Decoded value of `kCGImagePropertyDNGUniqueCameraModel`.
    pub unique_camera_model: Option<String>,
}

impl ProRawProperties {
    /// Reads RAW and DNG entries from an `ImageIO` property dictionary.
    pub fn from_properties(properties: &ImageProperties) -> Result<Self, ImageError> {
        let raw = properties.dictionary(RAW_DICTIONARY_KEY)?;
        let dng = properties.dictionary(DNG_DICTIONARY_KEY)?;
        let profile_name = dng
            .as_ref()
            .map_or(Ok(None), |props| props.string(PROFILE_NAME_KEY))?;
        let unique_camera_model = raw
            .as_ref()
            .map_or(Ok(None), |props| props.string(UNIQUE_CAMERA_MODEL_KEY))?
            .or(dng
                .as_ref()
                .map_or(Ok(None), |props| props.string(UNIQUE_CAMERA_MODEL_KEY))?);
        Ok(Self {
            has_raw_dictionary: raw.is_some(),
            has_dng_dictionary: dng.is_some(),
            profile_name,
            unique_camera_model,
        })
    }
}

/// Builder for synthetic RAW/DNG properties.
#[derive(Debug)]
pub struct ProRawBuilder {
    root: MutableProperties,
    raw: MutableProperties,
    dng: MutableProperties,
    pending_error: Option<ImageError>,
}

impl ProRawBuilder {
    /// Creates mutable RAW and DNG property dictionaries.
    pub fn new() -> Result<Self, ImageError> {
        Ok(Self {
            root: MutableProperties::new()?,
            raw: MutableProperties::new()?,
            dng: MutableProperties::new()?,
            pending_error: None,
        })
    }

    #[must_use]
    /// Sets `kCGImagePropertyDNGProfileName`.
    pub fn profile_name(mut self, profile_name: &str) -> Self {
        if self.pending_error.is_none() {
            self.pending_error = self.dng.set_string(PROFILE_NAME_KEY, profile_name).err();
        }
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyDNGUniqueCameraModel`.
    pub fn unique_camera_model(mut self, model: &str) -> Self {
        if self.pending_error.is_none() {
            self.pending_error = self.raw.set_string(UNIQUE_CAMERA_MODEL_KEY, model).err();
        }
        if self.pending_error.is_none() {
            self.pending_error = self.dng.set_string(UNIQUE_CAMERA_MODEL_KEY, model).err();
        }
        self
    }

    /// Freezes the RAW and DNG dictionaries for destination-side encode properties.
    pub fn build(mut self) -> Result<ImageProperties, ImageError> {
        if let Some(error) = self.pending_error {
            return Err(error);
        }
        self.root
            .set_dictionary(RAW_DICTIONARY_KEY, &self.raw.freeze()?)?;
        self.root
            .set_dictionary(DNG_DICTIONARY_KEY, &self.dng.freeze()?)?;
        self.root.freeze()
    }
}

fn is_dng_identifier(identifier: &str) -> bool {
    if identifier == DNG_TYPE_IDENTIFIER {
        return true;
    }
    bridge::cstring(identifier).is_ok_and(|identifier| unsafe {
        ffi::imageio_type_identifier_conforms_to_dng(identifier.as_ptr())
    })
}

#[must_use]
/// Returns canonical DNG source identifiers and identifiers conforming to DNG.
pub fn supported_source_identifiers() -> Vec<String> {
    ImageSource::type_identifiers()
        .into_iter()
        .filter(|identifier| is_dng_identifier(identifier))
        .collect()
}

#[must_use]
/// Returns canonical DNG destination identifiers and identifiers conforming to DNG.
pub fn supported_destination_identifiers() -> Vec<String> {
    ImageDestination::type_identifiers()
        .into_iter()
        .filter(|identifier| is_dng_identifier(identifier))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{is_dng_identifier, DNG_TYPE_IDENTIFIER, PROFILE_NAME_KEY};

    #[test]
    fn dng_profile_name_uses_native_key_value() {
        assert_eq!(PROFILE_NAME_KEY, "DNGProfileName");
    }

    #[test]
    fn dng_identifier_matching_is_exact_or_conformance_based() {
        assert!(is_dng_identifier(DNG_TYPE_IDENTIFIER));
        assert!(!is_dng_identifier("com.example.dng-preview"));
        assert!(!is_dng_identifier("com.example.proraw-thumbnail"));
    }
}
