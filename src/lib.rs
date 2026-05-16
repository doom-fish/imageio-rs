#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [ImageIO](https://developer.apple.com/documentation/imageio) framework
//! on macOS — read, write, and convert images in any format the OS
//! supports (PNG, JPEG, HEIC, TIFF, GIF, BMP, RAW, …).
//!
//! `ImageIO` is pure C, so this crate is **zero-Swift** — just thin
//! `extern "C"` declarations linked against the system frameworks.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod animation;
pub(crate) mod block_support;
pub mod error;
pub mod ffi;
pub mod image;
pub mod metadata;
pub mod source;
pub(crate) mod util;

pub use animation::{animate_image, animate_image_from_bytes};
pub use error::ImageError;
pub use image::{
    convert_format, copy_image_source, decode_bgra, decode_bgra_from_bytes, encode_bgra_to_bytes,
    read_metadata, DecodedImage, ImageFormat, ImageMetadata,
};
pub use metadata::{Metadata, MetadataTag, MutableMetadata};
pub use source::{ImageSource, SourceStatus};

/// Common imports.
pub mod prelude {
    pub use crate::animation::{animate_image, animate_image_from_bytes};
    pub use crate::error::ImageError;
    pub use crate::image::{
        convert_format, copy_image_source, decode_bgra, decode_bgra_from_bytes,
        encode_bgra_to_bytes, read_metadata, DecodedImage, ImageFormat, ImageMetadata,
    };
    pub use crate::metadata::{Metadata, MetadataTag, MutableMetadata};
    pub use crate::source::{ImageSource, SourceStatus};
}
