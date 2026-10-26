use crate::bridge::{self, Handle};
use crate::{ImageError, ImageFormat, ImageSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb8,
    Rgba8,
}

impl PixelFormat {
    #[must_use]
    pub const fn channels(self) -> usize {
        match self {
            Self::Rgb8 => 3,
            Self::Rgba8 => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedRgbaImage {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

unsafe extern "C" {
    fn imageio_source_create_rgba8_at_index(
        source: Handle,
        index: usize,
        max_width: usize,
        max_height: usize,
        max_bytes: usize,
        width: *mut usize,
        height: *mut usize,
        limit_exceeded: *mut bool,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    fn imageio_encode_pixels(
        pixels: *const u8,
        length: usize,
        width: usize,
        height: usize,
        channels: usize,
        type_identifier: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
}

impl ImageSource {
    pub fn decode_rgba8_at_index(&self, index: usize) -> Result<DecodedRgbaImage, ImageError> {
        let limits = self.decode_limits();
        let (max_width, max_height, max_bytes) = limits.bridge_values();
        let (mut width, mut height, mut limit_exceeded) = (0, 0, false);
        let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
            imageio_source_create_rgba8_at_index(
                self.as_raw(),
                index,
                max_width,
                max_height,
                max_bytes,
                &raw mut width,
                &raw mut height,
                &raw mut limit_exceeded,
                buffer,
                size,
            )
        });
        if limit_exceeded {
            return Err(ImageError::LimitExceeded {
                width,
                height,
                limits,
            });
        }
        if raw.is_null() {
            return Err(ImageError::DecodeFailed(message));
        }
        let rgba = bridge::copy_data(raw);
        if width
            .checked_mul(height)
            .and_then(|count| count.checked_mul(4))
            != Some(rgba.len())
        {
            return Err(ImageError::DecodeFailed("invalid RGBA8 pixel count".into()));
        }
        Ok(DecodedRgbaImage {
            width,
            height,
            rgba,
        })
    }
}

pub fn decode_rgba8_from_bytes(data: &[u8]) -> Result<DecodedRgbaImage, ImageError> {
    let source = ImageSource::from_bytes(data)?;
    if source.frame_count() == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    source.decode_rgba8_at_index(0)
}

pub fn encode_pixels_to_bytes(
    pixels: &[u8],
    width: usize,
    height: usize,
    pixel_format: PixelFormat,
    format: ImageFormat,
) -> Result<Vec<u8>, ImageError> {
    if width == 0
        || height == 0
        || width > isize::MAX.unsigned_abs()
        || height > isize::MAX.unsigned_abs()
        || pixels.len() > isize::MAX.unsigned_abs()
    {
        return Err(ImageError::EncodeFailed("invalid pixel dimensions".into()));
    }
    if width
        .checked_mul(height)
        .and_then(|count| count.checked_mul(pixel_format.channels()))
        != Some(pixels.len())
    {
        return Err(ImageError::EncodeFailed("invalid pixel count".into()));
    }
    let type_identifier = bridge::cstring(format.type_identifier())?;
    let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        imageio_encode_pixels(
            pixels.as_ptr(),
            pixels.len(),
            width,
            height,
            pixel_format.channels(),
            type_identifier.as_ptr(),
            buffer,
            size,
        )
    });
    if raw.is_null() {
        return Err(ImageError::EncodeFailed(message));
    }
    Ok(bridge::copy_data(raw))
}
