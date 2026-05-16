//! RAW, DNG, and Apple `ProRAW` property helpers.

use crate::destination::ImageDestination;
use crate::error::ImageError;
use crate::properties::{ImageProperties, MutableProperties};
use crate::source::ImageSource;

pub const RAW_DICTIONARY_KEY: &str = "{Raw}";
pub const DNG_DICTIONARY_KEY: &str = "{DNG}";
pub const PROFILE_NAME_KEY: &str = "ProfileName";
pub const UNIQUE_CAMERA_MODEL_KEY: &str = "UniqueCameraModel";

/// Typed RAW/DNG property view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProRawProperties {
    pub has_raw_dictionary: bool,
    pub has_dng_dictionary: bool,
    pub profile_name: Option<String>,
    pub unique_camera_model: Option<String>,
}

impl ProRawProperties {
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
}

impl ProRawBuilder {
    pub fn new() -> Result<Self, ImageError> {
        Ok(Self {
            root: MutableProperties::new()?,
            raw: MutableProperties::new()?,
            dng: MutableProperties::new()?,
        })
    }

    #[must_use]
    pub fn profile_name(mut self, profile_name: &str) -> Self {
        let _ = self.dng.set_string(PROFILE_NAME_KEY, profile_name);
        self
    }

    #[must_use]
    pub fn unique_camera_model(mut self, model: &str) -> Self {
        let _ = self.raw.set_string(UNIQUE_CAMERA_MODEL_KEY, model);
        let _ = self.dng.set_string(UNIQUE_CAMERA_MODEL_KEY, model);
        self
    }

    pub fn build(mut self) -> Result<ImageProperties, ImageError> {
        self.root.set_dictionary(RAW_DICTIONARY_KEY, &self.raw.freeze()?)?;
        self.root.set_dictionary(DNG_DICTIONARY_KEY, &self.dng.freeze()?)?;
        self.root.freeze()
    }
}

#[must_use]
pub fn supported_source_identifiers() -> Vec<String> {
    ImageSource::type_identifiers()
        .into_iter()
        .filter(|identifier| identifier.contains("dng") || identifier.contains("proraw"))
        .collect()
}

#[must_use]
pub fn supported_destination_identifiers() -> Vec<String> {
    ImageDestination::type_identifiers()
        .into_iter()
        .filter(|identifier| identifier.contains("dng") || identifier.contains("proraw"))
        .collect()
}
