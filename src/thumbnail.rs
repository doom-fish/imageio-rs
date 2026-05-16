//! Thumbnail generation helpers.

use crate::bridge::{self, thumbnail as ffi};
use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::source::ImageSource;

pub const THUMBNAIL_IMAGES_KEY: &str = "ThumbnailImages";

/// Options for `CGImageSourceCreateThumbnailAtIndex`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThumbnailOptions {
    pub max_pixel_size: usize,
    pub always_create: bool,
    pub transform: bool,
}

impl ThumbnailOptions {
    #[must_use]
    pub const fn new(max_pixel_size: usize) -> Self {
        Self {
            max_pixel_size,
            always_create: true,
            transform: true,
        }
    }
}

impl Default for ThumbnailOptions {
    fn default() -> Self {
        Self::new(256)
    }
}

pub fn create_thumbnail(
    source: &ImageSource,
    index: usize,
    options: ThumbnailOptions,
) -> Result<DecodedImage, ImageError> {
    let mut width = 0_usize;
    let mut height = 0_usize;
    let (raw, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_source_create_thumbnail_bgra_at_index(
            source.as_raw(),
            index,
            options.max_pixel_size,
            options.always_create,
            options.transform,
            &mut width,
            &mut height,
            buffer,
            size,
        )
    });
    if raw.is_null() {
        return Err(ImageError::DecodeFailed(if message.is_empty() {
            "imageio_source_create_thumbnail_bgra_at_index returned NULL".into()
        } else {
            message
        }));
    }
    Ok(DecodedImage {
        width,
        height,
        bgra: bridge::copy_data(raw),
    })
}
