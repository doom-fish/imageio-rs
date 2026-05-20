//! Errors returned by the `imageio` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Reports failures from safe wrappers around `ImageIO` entry points.
pub enum ImageError {
    /// The path could not be converted into the `CFURL` form used by `CGImageSourceCreateWithURL` or `CGImageDestinationCreateWithURL`.
    InvalidPath(String),
    /// Opening a `CGImageSource` failed.
    OpenSourceFailed(String),
    /// `CGImageSourceGetCount` reported that the source contains no images.
    NoImagesInSource,
    /// A decode-oriented `ImageIO` call failed.
    DecodeFailed(String),
    /// An encode-oriented `ImageIO` call failed.
    EncodeFailed(String),
    /// The requested type identifier is not supported by the active `ImageIO` codecs.
    UnsupportedFormat(String),
    /// `ImageIO` returned an error that does not fit a more specific category.
    Unknown(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath(m) => write!(f, "invalid path: {m}"),
            Self::OpenSourceFailed(m) => write!(f, "open source failed: {m}"),
            Self::NoImagesInSource => write!(f, "image source contains zero images"),
            Self::DecodeFailed(m) => write!(f, "decode failed: {m}"),
            Self::EncodeFailed(m) => write!(f, "encode failed: {m}"),
            Self::UnsupportedFormat(m) => write!(f, "unsupported format: {m}"),
            Self::Unknown(m) => write!(f, "imageio error: {m}"),
        }
    }
}

impl std::error::Error for ImageError {}

#[cfg(test)]
mod tests {
    use super::ImageError;

    #[test]
    fn display_formats_path_and_source_open_errors() {
        assert_eq!(
            ImageError::InvalidPath("bad path".to_owned()).to_string(),
            "invalid path: bad path"
        );
        assert_eq!(
            ImageError::OpenSourceFailed("decoder unavailable".to_owned()).to_string(),
            "open source failed: decoder unavailable"
        );
    }

    #[test]
    fn display_formats_decode_and_encode_errors() {
        assert_eq!(
            ImageError::DecodeFailed("bad pixels".to_owned()).to_string(),
            "decode failed: bad pixels"
        );
        assert_eq!(
            ImageError::EncodeFailed("writer rejected frame".to_owned()).to_string(),
            "encode failed: writer rejected frame"
        );
    }

    #[test]
    fn display_formats_unsupported_and_unknown_errors() {
        assert_eq!(
            ImageError::UnsupportedFormat("public.heics".to_owned()).to_string(),
            "unsupported format: public.heics"
        );
        assert_eq!(
            ImageError::Unknown("bridge returned NULL".to_owned()).to_string(),
            "imageio error: bridge returned NULL"
        );
    }

    #[test]
    fn no_images_in_source_has_specific_message() {
        assert_eq!(
            ImageError::NoImagesInSource.to_string(),
            "image source contains zero images"
        );
    }
}
