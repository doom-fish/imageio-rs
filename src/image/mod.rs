//! High-level image helpers built on top of `ImageIO` sources and destinations.

use std::path::Path;

use crate::destination::ImageDestination;
use crate::error::ImageError;
use crate::source::ImageSource;

/// Output type identifiers used by the convenience helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageFormat {
    /// Uses the PNG type identifier accepted by `CGImageDestinationCreateWithURL`.
    Png,
    /// Uses the JPEG type identifier accepted by `CGImageDestinationCreateWithURL`.
    Jpeg,
    /// Uses the HEIC type identifier accepted by `CGImageDestinationCreateWithURL`.
    Heic,
    /// Uses the HEIF type identifier accepted by `CGImageDestinationCreateWithURL`.
    Heif,
    /// Uses the HEICS sequence type identifier accepted by `CGImageDestinationCreateWithURL`.
    Heics,
    /// Uses the TIFF type identifier accepted by `CGImageDestinationCreateWithURL`.
    Tiff,
    /// Uses the GIF type identifier accepted by `CGImageDestinationCreateWithURL`.
    Gif,
    /// Uses the BMP type identifier accepted by `CGImageDestinationCreateWithURL`.
    Bmp,
    /// Uses the DNG type identifier accepted by `CGImageDestinationCreateWithURL`.
    Dng,
}

impl ImageFormat {
    #[must_use]
    /// Returns the destination type identifier passed to `CGImageDestinationCreateWithURL`.
    pub const fn type_identifier(self) -> &'static str {
        match self {
            Self::Png => "public.png",
            Self::Jpeg => "public.jpeg",
            Self::Heic => "public.heic",
            Self::Heif => "public.heif",
            Self::Heics => "public.heics",
            Self::Tiff => "public.tiff",
            Self::Gif => "com.compuserve.gif",
            Self::Bmp => "public.bmp",
            Self::Dng => "com.adobe.raw-image",
        }
    }
}

/// Tightly-packed BGRA image data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedImage {
    /// Pixel width reported by `CGImageGetWidth` for the decoded frame.
    pub width: usize,
    /// Pixel height reported by `CGImageGetHeight` for the decoded frame.
    pub height: usize,
    /// Tightly packed BGRA bytes decoded from `CGImageSourceCreateImageAtIndex`.
    pub bgra: Vec<u8>,
}

impl DecodedImage {
    #[must_use]
    /// Returns the row stride used for `CGBitmapContextCreate`.
    pub const fn bytes_per_row(&self) -> usize {
        self.width * 4
    }
}

/// Metadata read from an image source without forcing a full decode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageMetadata {
    /// Pixel width read from `kCGImagePropertyPixelWidth`.
    pub width: usize,
    /// Pixel height read from `kCGImagePropertyPixelHeight`.
    pub height: usize,
    /// Frame count reported by `CGImageSourceGetCount`.
    pub frame_count: usize,
    /// Whether `kCGImagePropertyHasAlpha` is set.
    pub has_alpha: bool,
    /// Source type identifier reported by `CGImageSourceGetType`.
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

/// Reads header metadata using `CGImageSourceCopyPropertiesAtIndex`.
pub fn read_metadata(path: impl AsRef<Path>) -> Result<ImageMetadata, ImageError> {
    let source = ImageSource::from_path(path)?;
    read_metadata_from_source(&source)
}

/// Decodes the first frame via `CGImageSourceCreateImageAtIndex`.
pub fn decode_bgra(path: impl AsRef<Path>) -> Result<DecodedImage, ImageError> {
    let source = ImageSource::from_path(path)?;
    if source.frame_count() == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    source.decode_image_at_index(0)
}

/// Decodes the first in-memory frame via `CGImageSourceCreateWithData`.
pub fn decode_bgra_from_bytes(data: &[u8]) -> Result<DecodedImage, ImageError> {
    let source = ImageSource::from_bytes(data)?;
    if source.frame_count() == 0 {
        return Err(ImageError::NoImagesInSource);
    }
    source.decode_image_at_index(0)
}

/// Encodes BGRA bytes with `CGImageDestinationCreateWithData`.
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

/// Re-encodes the first frame by pairing `CGImageSourceCreateWithURL` with `CGImageDestinationCreateWithURL`.
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

/// Copies a whole source with `CGImageDestinationCopyImageSource`.
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
    destination.copy_image_source(&source, None)
}

#[cfg(test)]
mod tests {
    use super::{DecodedImage, ImageFormat};

    #[test]
    fn image_format_identifiers_match_expected_values() {
        assert_eq!(ImageFormat::Png.type_identifier(), "public.png");
        assert_eq!(ImageFormat::Jpeg.type_identifier(), "public.jpeg");
        assert_eq!(ImageFormat::Heic.type_identifier(), "public.heic");
        assert_eq!(ImageFormat::Heif.type_identifier(), "public.heif");
        assert_eq!(ImageFormat::Heics.type_identifier(), "public.heics");
        assert_eq!(ImageFormat::Tiff.type_identifier(), "public.tiff");
        assert_eq!(ImageFormat::Gif.type_identifier(), "com.compuserve.gif");
        assert_eq!(ImageFormat::Bmp.type_identifier(), "public.bmp");
        assert_eq!(ImageFormat::Dng.type_identifier(), "com.adobe.raw-image");
    }

    #[test]
    fn decoded_image_bytes_per_row_matches_bgra_stride() {
        let image = DecodedImage {
            width: 3,
            height: 2,
            bgra: vec![0; 24],
        };

        assert_eq!(image.bytes_per_row(), 12);
    }
}
