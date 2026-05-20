//! Color-profile and HDR option helpers.

use crate::bridge::{self, color_sync as ffi};
use crate::error::ImageError;
use crate::properties::{ImageProperties, MutableProperties};
use crate::source::ImageSource;

/// Maps to the `kCGImagePropertyProfileName` entry.
pub const PROFILE_NAME_KEY: &str = "ProfileName";
/// Maps to the `kCGImageSourceDecodeRequest` option key.
pub const SOURCE_DECODE_REQUEST_KEY: &str = "kCGImageSourceDecodeRequest";
/// Maps to the `kCGImageSourceDecodeToHDR` decode request value.
pub const SOURCE_DECODE_TO_HDR: &str = "kCGImageSourceDecodeToHDR";
/// Maps to the `kCGImageSourceDecodeToSDR` decode request value.
pub const SOURCE_DECODE_TO_SDR: &str = "kCGImageSourceDecodeToSDR";
/// Maps to the `kCGImageSourceGenerateImageSpecificLumaScaling` option key.
pub const SOURCE_GENERATE_IMAGE_SPECIFIC_LUMA_SCALING_KEY: &str =
    "kCGImageSourceGenerateImageSpecificLumaScaling";
/// Maps to the `kCGImageDestinationOptimizeColorForSharing` option key.
pub const DESTINATION_OPTIMIZE_COLOR_FOR_SHARING_KEY: &str =
    "kCGImageDestinationOptimizeColorForSharing";
/// Maps to the `kCGImageDestinationPreserveGainMap` option key.
pub const DESTINATION_PRESERVE_GAIN_MAP_KEY: &str = "kCGImageDestinationPreserveGainMap";
/// Maps to the `kCGImageDestinationEncodeRequest` option key.
pub const DESTINATION_ENCODE_REQUEST_KEY: &str = "kCGImageDestinationEncodeRequest";
/// Maps to the `kCGImageDestinationEncodeToSDR` encode request value.
pub const DESTINATION_ENCODE_TO_SDR: &str = "kCGImageDestinationEncodeToSDR";
/// Maps to the `kCGImageDestinationEncodeToISOHDR` encode request value.
pub const DESTINATION_ENCODE_TO_ISO_HDR: &str = "kCGImageDestinationEncodeToISOHDR";
/// Maps to the `kCGImageDestinationEncodeToISOGainmap` encode request value.
pub const DESTINATION_ENCODE_TO_ISO_GAINMAP: &str = "kCGImageDestinationEncodeToISOGainmap";

/// Source decode request values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodeRequest {
    /// Requests the `kCGImageSourceDecodeToHDR` decode path.
    Hdr,
    /// Requests the `kCGImageSourceDecodeToSDR` decode path.
    Sdr,
}

impl DecodeRequest {
    /// Returns the `kCGImageSourceDecodeRequest` value for this mode.
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
    /// Requests the `kCGImageDestinationEncodeToSDR` encode path.
    Sdr,
    /// Requests the `kCGImageDestinationEncodeToISOHDR` encode path.
    IsoHdr,
    /// Requests the `kCGImageDestinationEncodeToISOGainmap` encode path.
    IsoGainMap,
}

impl EncodeRequest {
    /// Returns the `kCGImageDestinationEncodeRequest` value for this mode.
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
/// Reads the `kCGImagePropertyProfileName` value from a property dictionary.
pub fn profile_name(properties: &ImageProperties) -> Option<String> {
    bridge::copy_string(unsafe { ffi::imageio_properties_copy_profile_name(properties.as_raw()) })
}

/// Wraps the source-side profile-name lookup used with `CGImageSourceCopyPropertiesAtIndex`.
pub fn source_profile_name(
    source: &ImageSource,
    index: usize,
) -> Result<Option<String>, ImageError> {
    let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_source_copy_profile_name_at_index(source.as_raw(), index, buffer, size)
    });
    if raw.is_null() && !message.is_empty() {
        return Err(ImageError::DecodeFailed(message));
    }
    Ok(bridge::copy_string(raw))
}

/// Sets `kCGImageSourceDecodeRequest`.
pub fn set_decode_request(
    properties: &mut MutableProperties,
    request: DecodeRequest,
) -> Result<(), ImageError> {
    properties.set_string(SOURCE_DECODE_REQUEST_KEY, request.value())
}

/// Sets `kCGImageDestinationEncodeRequest`.
pub fn set_encode_request(
    properties: &mut MutableProperties,
    request: EncodeRequest,
) -> Result<(), ImageError> {
    properties.set_string(DESTINATION_ENCODE_REQUEST_KEY, request.value())
}

/// Sets `kCGImageSourceGenerateImageSpecificLumaScaling`.
pub fn set_generate_image_specific_luma_scaling(
    properties: &mut MutableProperties,
    enabled: bool,
) -> Result<(), ImageError> {
    properties.set_bool(SOURCE_GENERATE_IMAGE_SPECIFIC_LUMA_SCALING_KEY, enabled)
}

/// Sets `kCGImageDestinationOptimizeColorForSharing`.
pub fn set_optimize_color_for_sharing(
    properties: &mut MutableProperties,
    enabled: bool,
) -> Result<(), ImageError> {
    properties.set_bool(DESTINATION_OPTIMIZE_COLOR_FOR_SHARING_KEY, enabled)
}

/// Sets `kCGImageDestinationPreserveGainMap`.
pub fn set_preserve_gain_map(
    properties: &mut MutableProperties,
    enabled: bool,
) -> Result<(), ImageError> {
    properties.set_bool(DESTINATION_PRESERVE_GAIN_MAP_KEY, enabled)
}

#[cfg(test)]
mod tests {
    use super::{
        DecodeRequest, EncodeRequest, DESTINATION_ENCODE_REQUEST_KEY,
        DESTINATION_ENCODE_TO_ISO_GAINMAP, DESTINATION_ENCODE_TO_ISO_HDR,
        DESTINATION_ENCODE_TO_SDR, DESTINATION_OPTIMIZE_COLOR_FOR_SHARING_KEY,
        DESTINATION_PRESERVE_GAIN_MAP_KEY, PROFILE_NAME_KEY, SOURCE_DECODE_REQUEST_KEY,
        SOURCE_DECODE_TO_HDR, SOURCE_DECODE_TO_SDR,
        SOURCE_GENERATE_IMAGE_SPECIFIC_LUMA_SCALING_KEY,
    };

    #[test]
    fn decode_request_values_match_expected_constants() {
        assert_eq!(DecodeRequest::Hdr.value(), SOURCE_DECODE_TO_HDR);
        assert_eq!(DecodeRequest::Sdr.value(), SOURCE_DECODE_TO_SDR);
    }

    #[test]
    fn encode_request_values_match_expected_constants() {
        assert_eq!(EncodeRequest::Sdr.value(), DESTINATION_ENCODE_TO_SDR);
        assert_eq!(EncodeRequest::IsoHdr.value(), DESTINATION_ENCODE_TO_ISO_HDR);
        assert_eq!(EncodeRequest::IsoGainMap.value(), DESTINATION_ENCODE_TO_ISO_GAINMAP);
    }

    #[test]
    fn color_sync_property_keys_match_expected_names() {
        assert_eq!(PROFILE_NAME_KEY, "ProfileName");
        assert_eq!(SOURCE_DECODE_REQUEST_KEY, "kCGImageSourceDecodeRequest");
        assert_eq!(
            SOURCE_GENERATE_IMAGE_SPECIFIC_LUMA_SCALING_KEY,
            "kCGImageSourceGenerateImageSpecificLumaScaling"
        );
        assert_eq!(DESTINATION_ENCODE_REQUEST_KEY, "kCGImageDestinationEncodeRequest");
        assert_eq!(
            DESTINATION_OPTIMIZE_COLOR_FOR_SHARING_KEY,
            "kCGImageDestinationOptimizeColorForSharing"
        );
        assert_eq!(
            DESTINATION_PRESERVE_GAIN_MAP_KEY,
            "kCGImageDestinationPreserveGainMap"
        );
    }
}
