//! Color-profile and HDR option helpers.

use crate::bridge::{self, color_sync as ffi};
use crate::error::ImageError;
use crate::properties::{ImageProperties, MutableProperties};
use crate::source::ImageSource;

pub const PROFILE_NAME_KEY: &str = "ProfileName";
pub const SOURCE_DECODE_REQUEST_KEY: &str = "kCGImageSourceDecodeRequest";
pub const SOURCE_DECODE_TO_HDR: &str = "kCGImageSourceDecodeToHDR";
pub const SOURCE_DECODE_TO_SDR: &str = "kCGImageSourceDecodeToSDR";
pub const SOURCE_GENERATE_IMAGE_SPECIFIC_LUMA_SCALING_KEY: &str =
    "kCGImageSourceGenerateImageSpecificLumaScaling";
pub const DESTINATION_OPTIMIZE_COLOR_FOR_SHARING_KEY: &str =
    "kCGImageDestinationOptimizeColorForSharing";
pub const DESTINATION_PRESERVE_GAIN_MAP_KEY: &str = "kCGImageDestinationPreserveGainMap";
pub const DESTINATION_ENCODE_REQUEST_KEY: &str = "kCGImageDestinationEncodeRequest";
pub const DESTINATION_ENCODE_TO_SDR: &str = "kCGImageDestinationEncodeToSDR";
pub const DESTINATION_ENCODE_TO_ISO_HDR: &str = "kCGImageDestinationEncodeToISOHDR";
pub const DESTINATION_ENCODE_TO_ISO_GAINMAP: &str = "kCGImageDestinationEncodeToISOGainmap";

/// Source decode request values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeRequest {
    Hdr,
    Sdr,
}

impl DecodeRequest {
    #[must_use]
    pub const fn value(self) -> &'static str {
        match self {
            Self::Hdr => SOURCE_DECODE_TO_HDR,
            Self::Sdr => SOURCE_DECODE_TO_SDR,
        }
    }
}

/// Destination HDR encode request values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodeRequest {
    Sdr,
    IsoHdr,
    IsoGainMap,
}

impl EncodeRequest {
    #[must_use]
    pub const fn value(self) -> &'static str {
        match self {
            Self::Sdr => DESTINATION_ENCODE_TO_SDR,
            Self::IsoHdr => DESTINATION_ENCODE_TO_ISO_HDR,
            Self::IsoGainMap => DESTINATION_ENCODE_TO_ISO_GAINMAP,
        }
    }
}

#[must_use]
pub fn profile_name(properties: &ImageProperties) -> Option<String> {
    bridge::copy_string(unsafe { ffi::imageio_properties_copy_profile_name(properties.as_raw()) })
}

pub fn source_profile_name(source: &ImageSource, index: usize) -> Result<Option<String>, ImageError> {
    let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_source_copy_profile_name_at_index(source.as_raw(), index, buffer, size)
    });
    if raw.is_null() && !message.is_empty() {
        return Err(ImageError::DecodeFailed(message));
    }
    Ok(bridge::copy_string(raw))
}

pub fn set_decode_request(
    properties: &mut MutableProperties,
    request: DecodeRequest,
) -> Result<(), ImageError> {
    properties.set_string(SOURCE_DECODE_REQUEST_KEY, request.value())
}

pub fn set_encode_request(
    properties: &mut MutableProperties,
    request: EncodeRequest,
) -> Result<(), ImageError> {
    properties.set_string(DESTINATION_ENCODE_REQUEST_KEY, request.value())
}

pub fn set_generate_image_specific_luma_scaling(
    properties: &mut MutableProperties,
    enabled: bool,
) -> Result<(), ImageError> {
    properties.set_bool(SOURCE_GENERATE_IMAGE_SPECIFIC_LUMA_SCALING_KEY, enabled)
}

pub fn set_optimize_color_for_sharing(
    properties: &mut MutableProperties,
    enabled: bool,
) -> Result<(), ImageError> {
    properties.set_bool(DESTINATION_OPTIMIZE_COLOR_FOR_SHARING_KEY, enabled)
}

pub fn set_preserve_gain_map(
    properties: &mut MutableProperties,
    enabled: bool,
) -> Result<(), ImageError> {
    properties.set_bool(DESTINATION_PRESERVE_GAIN_MAP_KEY, enabled)
}
