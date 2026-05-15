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
//! ImageIO is pure C, so this crate is **zero-Swift** — just thin
//! `extern "C"` declarations linked against the system frameworks.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;
pub mod image;

pub use error::ImageError;
pub use image::{convert_format, decode_bgra, read_metadata, DecodedImage, ImageFormat, ImageMetadata};

/// Common imports.
pub mod prelude {
    pub use crate::error::ImageError;
    pub use crate::image::{
        convert_format, decode_bgra, read_metadata, DecodedImage, ImageFormat, ImageMetadata,
    };
}
