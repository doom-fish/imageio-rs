//! Auxiliary image data helpers.

use crate::bridge::{self, auxiliary_data as ffi, Handle};
use crate::error::ImageError;
use crate::metadata::Metadata;
use crate::properties::ImageProperties;

/// Auxiliary-data type identifiers supported by `ImageIO`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AuxiliaryDataType {
    /// Matches `kCGImageAuxiliaryDataTypeDepth`.
    Depth,
    /// Matches `kCGImageAuxiliaryDataTypeDisparity`.
    Disparity,
    /// Matches `kCGImageAuxiliaryDataTypePortraitEffectsMatte`.
    PortraitEffectsMatte,
    /// Matches `kCGImageAuxiliaryDataTypeSemanticSegmentationSkinMatte`.
    SemanticSegmentationSkinMatte,
    /// Matches `kCGImageAuxiliaryDataTypeSemanticSegmentationHairMatte`.
    SemanticSegmentationHairMatte,
    /// Matches `kCGImageAuxiliaryDataTypeSemanticSegmentationTeethMatte`.
    SemanticSegmentationTeethMatte,
    /// Matches `kCGImageAuxiliaryDataTypeSemanticSegmentationGlassesMatte`.
    SemanticSegmentationGlassesMatte,
    /// Matches `kCGImageAuxiliaryDataTypeSemanticSegmentationSkyMatte`.
    SemanticSegmentationSkyMatte,
    /// Matches `kCGImageAuxiliaryDataTypeHDRGainMap`.
    HdrGainMap,
    /// Matches `kCGImageAuxiliaryDataTypeISOGainMap`.
    IsoGainMap,
}

impl AuxiliaryDataType {
    #[must_use]
    /// Returns the `CGImageSourceCopyAuxiliaryDataInfoAtIndex` type identifier for this payload.
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::Depth => "kCGImageAuxiliaryDataTypeDepth",
            Self::Disparity => "kCGImageAuxiliaryDataTypeDisparity",
            Self::PortraitEffectsMatte => "kCGImageAuxiliaryDataTypePortraitEffectsMatte",
            Self::SemanticSegmentationSkinMatte => {
                "kCGImageAuxiliaryDataTypeSemanticSegmentationSkinMatte"
            }
            Self::SemanticSegmentationHairMatte => {
                "kCGImageAuxiliaryDataTypeSemanticSegmentationHairMatte"
            }
            Self::SemanticSegmentationTeethMatte => {
                "kCGImageAuxiliaryDataTypeSemanticSegmentationTeethMatte"
            }
            Self::SemanticSegmentationGlassesMatte => {
                "kCGImageAuxiliaryDataTypeSemanticSegmentationGlassesMatte"
            }
            Self::SemanticSegmentationSkyMatte => {
                "kCGImageAuxiliaryDataTypeSemanticSegmentationSkyMatte"
            }
            Self::HdrGainMap => "kCGImageAuxiliaryDataTypeHDRGainMap",
            Self::IsoGainMap => "kCGImageAuxiliaryDataTypeISOGainMap",
        }
    }
}

/// Owned auxiliary-data payload.
#[derive(Debug)]
pub struct AuxiliaryDataInfo {
    raw: Handle,
}

impl AuxiliaryDataInfo {
    pub(crate) fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> Handle {
        self.raw
    }

    /// Creates an owned auxiliary-data info payload for `CGImageDestinationAddAuxiliaryDataInfo`.
    pub fn new() -> Result<Self, ImageError> {
        Self::from_raw(unsafe { ffi::imageio_auxiliary_data_info_create() }).ok_or_else(|| {
            ImageError::Unknown("imageio_auxiliary_data_info_create returned NULL".into())
        })
    }

    /// Sets the raw auxiliary payload bytes.
    pub fn set_data(&mut self, data: &[u8]) {
        unsafe { ffi::imageio_auxiliary_data_info_set_data(self.raw, data.as_ptr(), data.len()) };
    }

    #[must_use]
    /// Returns the raw auxiliary payload bytes.
    pub fn data(&self) -> Vec<u8> {
        bridge::copy_data(unsafe { ffi::imageio_auxiliary_data_info_copy_data(self.raw) })
    }

    /// Sets the auxiliary-data description dictionary returned by `CGImageSourceCopyAuxiliaryDataInfoAtIndex`.
    pub fn set_description(&mut self, properties: &ImageProperties) {
        unsafe { ffi::imageio_auxiliary_data_info_set_description(self.raw, properties.as_raw()) };
    }

    #[must_use]
    /// Returns the auxiliary-data description dictionary.
    pub fn description(&self) -> Option<ImageProperties> {
        ImageProperties::from_raw(unsafe {
            ffi::imageio_auxiliary_data_info_copy_description(self.raw)
        })
    }

    /// Sets the metadata tree associated with this auxiliary payload.
    pub fn set_metadata(&mut self, metadata: &Metadata) {
        unsafe { ffi::imageio_auxiliary_data_info_set_metadata(self.raw, metadata.as_raw()) };
    }

    #[must_use]
    /// Returns the metadata tree associated with this auxiliary payload.
    pub fn metadata(&self) -> Option<Metadata> {
        Metadata::from_raw(unsafe { ffi::imageio_auxiliary_data_info_copy_metadata(self.raw) })
    }

    #[must_use]
    /// Reports whether the auxiliary payload includes color-space information.
    pub fn has_color_space(&self) -> bool {
        unsafe { ffi::imageio_auxiliary_data_info_has_color_space(self.raw) }
    }
}

impl Clone for AuxiliaryDataInfo {
    fn clone(&self) -> Self {
        Self {
            raw: bridge::retain(self.raw),
        }
    }
}

impl Drop for AuxiliaryDataInfo {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}
