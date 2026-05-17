//! Safe wrapper around `CGImageDestination`.

use std::path::Path;

use crate::auxiliary_data::{AuxiliaryDataInfo, AuxiliaryDataType};
use crate::bridge::{self, destination as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::metadata::Metadata;
use crate::properties::ImageProperties;
use crate::source::ImageSource;

/// Owned destination handle.
#[derive(Debug)]
pub struct ImageDestination {
    raw: Handle,
}

impl ImageDestination {
    fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self { raw })
    }

    #[must_use]
    pub fn type_identifiers() -> Vec<String> {
        bridge::copy_string_array(unsafe { ffi::imageio_destination_copy_type_identifiers() })
    }

    pub fn to_path(
        path: impl AsRef<Path>,
        type_identifier: &str,
        image_count: usize,
    ) -> Result<Self, ImageError> {
        let path = bridge::path_to_cstring(path.as_ref())?;
        let type_identifier = bridge::cstring(type_identifier)?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_create_with_url(
                path.as_ptr(),
                type_identifier.as_ptr(),
                image_count,
                buffer,
                size,
            )
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_create_with_url returned NULL".into()
            } else {
                message
            })
        })
    }

    pub fn to_data(type_identifier: &str, image_count: usize) -> Result<Self, ImageError> {
        let type_identifier = bridge::cstring(type_identifier)?;
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_create_with_data(
                type_identifier.as_ptr(),
                image_count,
                buffer,
                size,
            )
        });
        Self::from_raw(raw).ok_or_else(|| {
            ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_create_with_data returned NULL".into()
            } else {
                message
            })
        })
    }

    pub fn set_properties(&mut self, properties: &ImageProperties) {
        unsafe { ffi::imageio_destination_set_properties(self.raw, properties.as_raw()) };
    }

    pub fn add_image(
        &mut self,
        image: &DecodedImage,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        let expected = image.width.saturating_mul(image.height).saturating_mul(4);
        if image.bgra.len() < expected {
            return Err(ImageError::EncodeFailed(format!(
                "buffer too small for {}x{} BGRA image",
                image.width, image.height
            )));
        }
        let properties_raw = properties.map_or(std::ptr::null_mut(), ImageProperties::as_raw);
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_add_bgra_image(
                self.raw,
                image.bgra.as_ptr(),
                image.bgra.len(),
                image.width,
                image.height,
                properties_raw,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_add_bgra_image returned false".into()
            } else {
                message
            }))
        }
    }

    pub fn add_image_with_metadata(
        &mut self,
        image: &DecodedImage,
        metadata: &Metadata,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        let properties_raw = properties.map_or(std::ptr::null_mut(), ImageProperties::as_raw);
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_add_bgra_image_with_metadata(
                self.raw,
                image.bgra.as_ptr(),
                image.bgra.len(),
                image.width,
                image.height,
                metadata.as_raw(),
                properties_raw,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_add_bgra_image_with_metadata returned false".into()
            } else {
                message
            }))
        }
    }

    pub fn add_image_from_source(
        &mut self,
        source: &ImageSource,
        index: usize,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        let properties_raw = properties.map_or(std::ptr::null_mut(), ImageProperties::as_raw);
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_add_image_from_source(
                self.raw,
                source.as_raw(),
                index,
                properties_raw,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_add_image_from_source returned false".into()
            } else {
                message
            }))
        }
    }

    pub fn copy_image_source(
        &mut self,
        source: &ImageSource,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        let properties_raw = properties.map_or(std::ptr::null_mut(), ImageProperties::as_raw);
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_copy_image_source(
                self.raw,
                source.as_raw(),
                properties_raw,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_copy_image_source returned false".into()
            } else {
                message
            }))
        }
    }

    pub fn add_auxiliary_data_info(
        &mut self,
        auxiliary_type: AuxiliaryDataType,
        info: &AuxiliaryDataInfo,
    ) -> Result<(), ImageError> {
        let auxiliary_type = bridge::cstring(auxiliary_type.identifier())?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_add_auxiliary_data_info(
                self.raw,
                auxiliary_type.as_ptr(),
                info.as_raw(),
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_add_auxiliary_data_info returned false".into()
            } else {
                message
            }))
        }
    }

    pub fn finalize(&mut self) -> Result<(), ImageError> {
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_finalize(self.raw, buffer, size)
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_finalize returned false".into()
            } else {
                message
            }))
        }
    }

    #[must_use]
    pub fn data(&self) -> Option<Vec<u8>> {
        let data = unsafe { ffi::imageio_destination_copy_data(self.raw) };
        (!data.is_null()).then(|| bridge::copy_data(data))
    }
}

impl Drop for ImageDestination {
    fn drop(&mut self) {
        bridge::release(self.raw);
    }
}
