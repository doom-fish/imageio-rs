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
