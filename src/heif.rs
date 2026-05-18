//! HEIF and HEICS property helpers.

use crate::destination::ImageDestination;
use crate::error::ImageError;
use crate::properties::{ImageProperties, MutableProperties};
use crate::source::ImageSource;

/// Maps to the `kCGImagePropertyHEIFDictionary` property dictionary.
pub const HEIF_DICTIONARY_KEY: &str = "{HEIF}";
/// Maps to the `kCGImagePropertyHEICSDictionary` property dictionary.
pub const HEICS_DICTIONARY_KEY: &str = "{HEICS}";
/// Maps to the `kCGImagePropertyHEIFPrimaryImage` entry.
pub const PRIMARY_IMAGE_KEY: &str = "PrimaryImage";
/// Maps to the `kCGImagePropertyHEICSLoopCount` entry.
pub const HEICS_LOOP_COUNT_KEY: &str = "LoopCount";
/// Maps to the `kCGImagePropertyHEICSDelayTime` entry.
pub const HEICS_DELAY_TIME_KEY: &str = "DelayTime";
/// Maps to the `kCGImagePropertyHEICSUnclampedDelayTime` entry.
pub const HEICS_UNCLAMPED_DELAY_TIME_KEY: &str = "UnclampedDelayTime";
/// Maps to the `kCGImagePropertyHEICSCanvasPixelWidth` entry.
pub const HEICS_CANVAS_PIXEL_WIDTH_KEY: &str = "CanvasPixelWidth";
/// Maps to the `kCGImagePropertyHEICSCanvasPixelHeight` entry.
pub const HEICS_CANVAS_PIXEL_HEIGHT_KEY: &str = "CanvasPixelHeight";

/// Typed HEIF/HEICS property view.
#[derive(Debug, Clone, PartialEq)]
pub struct HeifProperties {
    /// Decoded value of `kCGImagePropertyHEIFPrimaryImage`.
    pub is_primary: Option<bool>,
    /// Decoded value of `kCGImagePropertyHEICSLoopCount`.
    pub loop_count: Option<i64>,
    /// Decoded value of `kCGImagePropertyHEICSDelayTime`.
    pub delay_time: Option<f64>,
    /// Decoded value of `kCGImagePropertyHEICSUnclampedDelayTime`.
    pub unclamped_delay_time: Option<f64>,
    /// Decoded value of `kCGImagePropertyHEICSCanvasPixelWidth`.
    pub canvas_pixel_width: Option<i64>,
    /// Decoded value of `kCGImagePropertyHEICSCanvasPixelHeight`.
    pub canvas_pixel_height: Option<i64>,
}

impl HeifProperties {
    /// Reads HEIF and HEICS entries from an `ImageIO` property dictionary.
    pub fn from_properties(properties: &ImageProperties) -> Result<Self, ImageError> {
        let heics = properties.dictionary(HEICS_DICTIONARY_KEY)?;
        Ok(Self {
            is_primary: properties.bool(PRIMARY_IMAGE_KEY)?,
            loop_count: heics
                .as_ref()
                .map_or(Ok(None), |props| props.i64(HEICS_LOOP_COUNT_KEY))?,
            delay_time: heics
                .as_ref()
                .map_or(Ok(None), |props| props.f64(HEICS_DELAY_TIME_KEY))?,
            unclamped_delay_time: heics
                .as_ref()
                .map_or(Ok(None), |props| props.f64(HEICS_UNCLAMPED_DELAY_TIME_KEY))?,
            canvas_pixel_width: heics
                .as_ref()
                .map_or(Ok(None), |props| props.i64(HEICS_CANVAS_PIXEL_WIDTH_KEY))?,
            canvas_pixel_height: heics
                .as_ref()
                .map_or(Ok(None), |props| props.i64(HEICS_CANVAS_PIXEL_HEIGHT_KEY))?,
        })
    }
}

/// Builder for synthetic HEIF/HEICS properties.
#[derive(Debug)]
pub struct HeifBuilder {
    root: MutableProperties,
    heics: MutableProperties,
}

impl HeifBuilder {
    /// Creates mutable HEIF and HEICS property dictionaries.
    pub fn new() -> Result<Self, ImageError> {
        Ok(Self {
            root: MutableProperties::new()?,
            heics: MutableProperties::new()?,
        })
    }

    #[must_use]
    /// Sets `kCGImagePropertyHEIFPrimaryImage`.
    pub fn primary_image(mut self, is_primary: bool) -> Self {
        let _ = self.root.set_bool(PRIMARY_IMAGE_KEY, is_primary);
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyHEICSLoopCount`.
    pub fn loop_count(mut self, loop_count: i64) -> Self {
        let _ = self.heics.set_i64(HEICS_LOOP_COUNT_KEY, loop_count);
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyHEICSDelayTime`.
    pub fn delay_time(mut self, delay_time: f64) -> Self {
        let _ = self.heics.set_f64(HEICS_DELAY_TIME_KEY, delay_time);
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyHEICSCanvasPixelWidth` and `kCGImagePropertyHEICSCanvasPixelHeight`.
    pub fn canvas_size(mut self, width: i64, height: i64) -> Self {
        let _ = self.heics.set_i64(HEICS_CANVAS_PIXEL_WIDTH_KEY, width);
        let _ = self.heics.set_i64(HEICS_CANVAS_PIXEL_HEIGHT_KEY, height);
        self
    }

    /// Freezes the HEIF dictionaries for destination-side encode properties.
    pub fn build(mut self) -> Result<ImageProperties, ImageError> {
        self.root
            .set_dictionary(HEICS_DICTIONARY_KEY, &self.heics.freeze()?)?;
        self.root.freeze()
    }
}

#[must_use]
/// Returns `CGImageSourceCopyTypeIdentifiers` entries that look like HEIF or HEIC.
pub fn supported_source_identifiers() -> Vec<String> {
    ImageSource::type_identifiers()
        .into_iter()
        .filter(|identifier| identifier.contains("heic") || identifier.contains("heif"))
        .collect()
}

#[must_use]
/// Returns `CGImageDestinationCopyTypeIdentifiers` entries that look like HEIF or HEIC.
pub fn supported_destination_identifiers() -> Vec<String> {
    ImageDestination::type_identifiers()
        .into_iter()
        .filter(|identifier| identifier.contains("heic") || identifier.contains("heif"))
        .collect()
}
