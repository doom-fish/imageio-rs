//! APNG-specific property helpers.

use crate::error::ImageError;
use crate::properties::{ImageProperties, MutableProperties};

/// Maps to the `kCGImagePropertyPNGDictionary` property dictionary.
pub const PNG_DICTIONARY_KEY: &str = "{PNG}";
/// Maps to the `kCGImagePropertyAPNGLoopCount` entry.
pub const APNG_LOOP_COUNT_KEY: &str = "LoopCount";
/// Maps to the `kCGImagePropertyAPNGDelayTime` entry.
pub const APNG_DELAY_TIME_KEY: &str = "DelayTime";
/// Maps to the `kCGImagePropertyAPNGUnclampedDelayTime` entry.
pub const APNG_UNCLAMPED_DELAY_TIME_KEY: &str = "UnclampedDelayTime";
/// Maps to the `kCGImagePropertyAPNGCanvasPixelWidth` entry.
pub const APNG_CANVAS_PIXEL_WIDTH_KEY: &str = "CanvasPixelWidth";
/// Maps to the `kCGImagePropertyAPNGCanvasPixelHeight` entry.
pub const APNG_CANVAS_PIXEL_HEIGHT_KEY: &str = "CanvasPixelHeight";

/// Typed APNG property view.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedPngProperties {
    /// Decoded value of `kCGImagePropertyAPNGLoopCount`.
    pub loop_count: Option<i64>,
    /// Decoded value of `kCGImagePropertyAPNGDelayTime`.
    pub delay_time: Option<f64>,
    /// Decoded value of `kCGImagePropertyAPNGUnclampedDelayTime`.
    pub unclamped_delay_time: Option<f64>,
    /// Decoded value of `kCGImagePropertyAPNGCanvasPixelWidth`.
    pub canvas_pixel_width: Option<i64>,
    /// Decoded value of `kCGImagePropertyAPNGCanvasPixelHeight`.
    pub canvas_pixel_height: Option<i64>,
}

impl AnimatedPngProperties {
    /// Reads APNG entries from `kCGImagePropertyPNGDictionary`.
    pub fn from_properties(properties: &ImageProperties) -> Result<Option<Self>, ImageError> {
        let Some(png) = properties.dictionary(PNG_DICTIONARY_KEY)? else {
            return Ok(None);
        };
        Ok(Some(Self {
            loop_count: png.i64(APNG_LOOP_COUNT_KEY)?,
            delay_time: png.f64(APNG_DELAY_TIME_KEY)?,
            unclamped_delay_time: png.f64(APNG_UNCLAMPED_DELAY_TIME_KEY)?,
            canvas_pixel_width: png.i64(APNG_CANVAS_PIXEL_WIDTH_KEY)?,
            canvas_pixel_height: png.i64(APNG_CANVAS_PIXEL_HEIGHT_KEY)?,
        }))
    }
}

/// Builder for synthetic APNG properties.
#[derive(Debug)]
pub struct AnimatedPngBuilder {
    png: MutableProperties,
}

impl AnimatedPngBuilder {
    /// Creates a mutable `kCGImagePropertyPNGDictionary` builder.
    pub fn new() -> Result<Self, ImageError> {
        Ok(Self {
            png: MutableProperties::new()?,
        })
    }

    #[must_use]
    /// Sets `kCGImagePropertyAPNGLoopCount`.
    pub fn loop_count(mut self, loop_count: i64) -> Self {
        let _ = self.png.set_i64(APNG_LOOP_COUNT_KEY, loop_count);
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyAPNGDelayTime`.
    pub fn delay_time(mut self, delay_time: f64) -> Self {
        let _ = self.png.set_f64(APNG_DELAY_TIME_KEY, delay_time);
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyAPNGUnclampedDelayTime`.
    pub fn unclamped_delay_time(mut self, delay_time: f64) -> Self {
        let _ = self.png.set_f64(APNG_UNCLAMPED_DELAY_TIME_KEY, delay_time);
        self
    }

    #[must_use]
    /// Sets `kCGImagePropertyAPNGCanvasPixelWidth` and `kCGImagePropertyAPNGCanvasPixelHeight`.
    pub fn canvas_size(mut self, width: i64, height: i64) -> Self {
        let _ = self.png.set_i64(APNG_CANVAS_PIXEL_WIDTH_KEY, width);
        let _ = self.png.set_i64(APNG_CANVAS_PIXEL_HEIGHT_KEY, height);
        self
    }

    /// Freezes the APNG dictionary for `CGImageDestinationAddImage`.
    pub fn build(self) -> Result<ImageProperties, ImageError> {
        let mut root = MutableProperties::new()?;
        root.set_dictionary(PNG_DICTIONARY_KEY, &self.png.freeze()?)?;
        root.freeze()
    }
}
