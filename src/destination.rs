//! Safe wrapper around `CGImageDestination`.

use std::path::Path;

use crate::auxiliary_data::{AuxiliaryDataInfo, AuxiliaryDataType};
use crate::bridge::{self, destination as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::metadata::Metadata;
use crate::properties::ImageProperties;
use crate::source::ImageSource;

/// Re-exports Apple's `CGImage` for direct `CGImageDestinationAddImage` interop.
pub use apple_cf::cg::CGImage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DestinationState {
    Open,
    Complete,
}

/// Owned destination handle.
#[derive(Debug)]
pub struct ImageDestination {
    raw: Handle,
    state: DestinationState,
}

impl ImageDestination {
    fn from_raw(raw: Handle) -> Option<Self> {
        (!raw.is_null()).then_some(Self {
            raw,
            state: DestinationState::Open,
        })
    }

    fn ensure_open(&self, operation: &str) -> Result<(), ImageError> {
        if self.state == DestinationState::Open {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(format!(
                "image destination is already complete; cannot {operation}"
            )))
        }
    }

    fn validate_bgra_image(image: &DecodedImage) -> Result<(), ImageError> {
        if image.width == 0 || image.height == 0 {
            return Err(ImageError::EncodeFailed(
                "BGRA image dimensions must be non-zero".into(),
            ));
        }
        isize::try_from(image.width).map_err(|_| {
            ImageError::EncodeFailed("BGRA image width does not fit Swift Int".into())
        })?;
        isize::try_from(image.height).map_err(|_| {
            ImageError::EncodeFailed("BGRA image height does not fit Swift Int".into())
        })?;
        let bytes_per_row = image.width.checked_mul(4).ok_or_else(|| {
            ImageError::EncodeFailed("BGRA image row stride overflows usize".into())
        })?;
        isize::try_from(bytes_per_row).map_err(|_| {
            ImageError::EncodeFailed("BGRA image row stride does not fit Swift Int".into())
        })?;
        let expected = bytes_per_row.checked_mul(image.height).ok_or_else(|| {
            ImageError::EncodeFailed("BGRA image byte length overflows usize".into())
        })?;
        isize::try_from(expected).map_err(|_| {
            ImageError::EncodeFailed("BGRA image byte length does not fit Swift Int".into())
        })?;
        isize::try_from(image.bgra.len()).map_err(|_| {
            ImageError::EncodeFailed("BGRA buffer length does not fit Swift Int".into())
        })?;
        if image.bgra.len() < expected {
            return Err(ImageError::EncodeFailed(format!(
                "buffer too small for {}x{} BGRA image",
                image.width, image.height
            )));
        }
        Ok(())
    }

    #[must_use]
    /// Wraps `CGImageDestinationCopyTypeIdentifiers`.
    pub fn type_identifiers() -> Vec<String> {
        bridge::copy_string_array(unsafe { ffi::imageio_destination_copy_type_identifiers() })
    }

    /// Wraps `CGImageDestinationCreateWithURL`.
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

    /// Wraps `CGImageDestinationCreateWithData`.
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

    /// Wraps `CGImageDestinationSetProperties`.
    pub fn set_properties(&mut self, properties: &ImageProperties) -> Result<(), ImageError> {
        self.ensure_open("set properties")?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_set_properties(self.raw, properties.as_raw(), buffer, size)
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_set_properties returned false".into()
            } else {
                message
            }))
        }
    }

    /// Adds a BGRA frame via `CGImageDestinationAddImage`.
    pub fn add_image(
        &mut self,
        image: &DecodedImage,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        self.ensure_open("add an image")?;
        Self::validate_bgra_image(image)?;
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

    /// Adds a BGRA frame and `CGImageMetadataRef` via `CGImageDestinationAddImageAndMetadata`.
    pub fn add_image_with_metadata(
        &mut self,
        image: &DecodedImage,
        metadata: &Metadata,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        self.ensure_open("add an image")?;
        Self::validate_bgra_image(image)?;
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

    /// Add a [`CGImage`] directly to the destination without round-tripping
    /// through host BGRA bytes.
    ///
    /// Useful when the caller already holds a `CGImage` — e.g. one decoded
    /// from an `ImageSource`, produced by `VTCreateCGImageFromCVPixelBuffer`,
    /// or returned by a screen-capture API. Skips one decode-encode cycle and
    /// lets the OS preserve the native pixel format (e.g. YCbCr 4:2:0)
    /// end-to-end into output formats that support it natively (JPEG, HEIC) —
    /// no host-side colour conversion, no extra allocation.
    ///
    /// The destination takes its own reference via
    /// `CGImageDestinationAddImage`; the caller's [`CGImage`] is borrowed
    /// for the duration of the call and remains valid afterwards.
    ///
    /// # Errors
    ///
    /// Returns [`ImageError::EncodeFailed`] if the destination rejects the
    /// image (e.g. the destination's type identifier doesn't accept the
    /// image's pixel format).
    pub fn add_cg_image(
        &mut self,
        cg_image: &CGImage,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        self.ensure_open("add an image")?;
        let properties_raw = properties.map_or(std::ptr::null_mut(), ImageProperties::as_raw);
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_add_cg_image(
                self.raw,
                cg_image.as_ptr(),
                properties_raw,
                buffer,
                size,
            )
        });
        if ok {
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_add_cg_image returned false".into()
            } else {
                message
            }))
        }
    }

    /// Wraps `CGImageDestinationAddImageFromSource`.
    pub fn add_image_from_source(
        &mut self,
        source: &ImageSource,
        index: usize,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        self.ensure_open("add an image")?;
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

    /// Wraps `CGImageDestinationCopyImageSource`.
    pub fn copy_image_source(
        &mut self,
        source: &ImageSource,
        properties: Option<&ImageProperties>,
    ) -> Result<(), ImageError> {
        self.ensure_open("copy an image source")?;
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
            self.state = DestinationState::Complete;
            Ok(())
        } else {
            Err(ImageError::EncodeFailed(if message.is_empty() {
                "imageio_destination_copy_image_source returned false".into()
            } else {
                message
            }))
        }
    }

    /// Wraps `CGImageDestinationAddAuxiliaryDataInfo`.
    pub fn add_auxiliary_data_info(
        &mut self,
        auxiliary_type: AuxiliaryDataType,
        info: &AuxiliaryDataInfo,
    ) -> Result<(), ImageError> {
        self.ensure_open("add auxiliary data")?;
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

    /// Wraps `CGImageDestinationFinalize`.
    pub fn finalize(&mut self) -> Result<(), ImageError> {
        self.ensure_open("finalize")?;
        let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            ffi::imageio_destination_finalize(self.raw, buffer, size)
        });
        if ok {
            self.state = DestinationState::Complete;
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
    /// Returns the in-memory output produced by `CGImageDestinationCreateWithData`.
    pub fn data(&self) -> Option<Vec<u8>> {
        let data = unsafe { ffi::imageio_destination_copy_data(self.raw) };
        (!data.is_null()).then(|| bridge::copy_data(data))
    }
}

crate::bridge::retained::imageio_retained!(ImageDestination, drop_only);
