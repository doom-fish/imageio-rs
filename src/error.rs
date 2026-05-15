//! Errors returned by the `imageio` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImageError {
    InvalidPath(String),
    OpenSourceFailed(String),
    NoImagesInSource,
    DecodeFailed(String),
    EncodeFailed(String),
    UnsupportedFormat(String),
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
