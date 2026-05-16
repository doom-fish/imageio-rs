//! High-level image helpers built on top of `ImageIO` sources and destinations.

use std::path::Path;

use crate::destination::ImageDestination;
use crate::error::ImageError;
use crate::source::ImageSource;

/// Output type identifiers used by the convenience helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageFormat {
    Png,
    Jpeg,
    Heic,
    Heif,
    Heics,
    Tiff,
    Gif,
    Bmp,
    Dng,
}

impl ImageFormat {
    #[must_use]
    pub const fn type_identifier(self) -> &'static str {
        match self {
            Self::Png => "public.png",
            Self::Jpeg => "public.jpeg",
            Self::Heic => "public.heic",
            Self::Heif => "public.heif",
            Self::Heics => "public.heics",
            Self::Tiff => "public.tiff",
            Self::Gif => "com.compuserve.gif",
            Self::Bmp => "com.microsoft.bmp",
            Self::Dng => "com.adobe.raw-image",
        }
    }
}

/// Tightly-packed BGRA image data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedImage {
    pub width: usize,
    pub height: usize,
    pub bgra: Vec<u8>,
}

impl DecodedImage {
    #[must_use]
    pub const fn bytes_per_row(&self) -> usize {
        self.width * 4
    }
}

/// Metadata read from an image source without forcing a full decode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageMetadata {
    pub width: usize,
    pub height: usize,
    pub frame_count: usize,
    pub has_alpha: bool,
    pub source_format: Option<String>,
}

fn read_metadata_from_source(source: &ImageSource) -> Result<ImageMetadata, ImageError> {
    let frame_count = source.frame_count();
    if frame_count == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    let properties = source.properties_at_index(0)?;
    let width = properties
        .i64("PixelWidth")?
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(0);
    let height = properties
        .i64("PixelHeight")?
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(0);
    let has_alpha = properties.bool("HasAlpha")?.unwrap_or(false);
    Ok(ImageMetadata {
        width,
        height,
        frame_count,
        has_alpha,
        source_format: source.source_type(),
    })
}

pub fn read_metadata(path: impl AsRef<Path>) -> Result<ImageMetadata, ImageError> {
    let source = ImageSource::from_path(path)?;
    read_metadata_from_source(&source)
}

pub fn decode_bgra(path: impl AsRef<Path>) -> Result<DecodedImage, ImageError> {
    let source = ImageSource::from_path(path)?;
    if source.frame_count() == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    source.decode_image_at_index(0)
}

pub fn decode_bgra_from_bytes(data: &[u8]) -> Result<DecodedImage, ImageError> {
    let source = ImageSource::from_bytes(data)?;
    if source.frame_count() == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    source.decode_image_at_index(0)
}

pub fn encode_bgra_to_bytes(
    bgra: &[u8],
    width: usize,
    height: usize,
    format: ImageFormat,
) -> Result<Vec<u8>, ImageError> {
    let image = DecodedImage {
        width,
        height,
        bgra: bgra.to_vec(),
    };
    let mut destination = ImageDestination::to_data(format.type_identifier(), 1)?;
    destination.add_image(&image, None)?;
    destination.finalize()?;
    destination.data().ok_or_else(|| {
        ImageError::EncodeFailed("ImageDestination did not produce output bytes".into())
    })
}

pub fn convert_format(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: ImageFormat,
) -> Result<(), ImageError> {
    let source = ImageSource::from_path(input)?;
    if source.frame_count() == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    let mut destination = ImageDestination::to_path(output, format.type_identifier(), 1)?;
    destination.add_image_from_source(&source, 0, None)?;
    destination.finalize()
}

pub fn copy_image_source(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: Option<ImageFormat>,
) -> Result<(), ImageError> {
    let source = ImageSource::from_path(input)?;
    let frame_count = source.frame_count();
    if frame_count == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    let destination_type = if let Some(format) = format {
        format.type_identifier().to_string()
    } else {
        source.source_type().ok_or_else(|| {
            ImageError::UnsupportedFormat("image source does not report a type identifier".into())
        })?
    };
    let mut destination = ImageDestination::to_path(output, &destination_type, frame_count)?;
    destination.copy_image_source(&source, None)?;
    destination.finalize()
}
