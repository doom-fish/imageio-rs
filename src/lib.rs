#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

//! Safe Rust bindings for Apple's `ImageIO` framework on macOS.
//! The default build talks to `ImageIO` through a Swift bridge and exposes
//! safe wrappers for sources, destinations, thumbnails, metadata, HEIF,
//! APNG, auxiliary data, and color-profile helpers.

pub mod animated_png;
pub mod animation;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
pub mod auxiliary_data;
pub(crate) mod bridge;
pub mod color_sync;
pub mod destination;
pub mod error;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod ffi;
pub mod heif;
pub mod image;
pub mod metadata;
pub mod properties;
pub mod proraw;
pub mod source;
pub mod thumbnail;

pub use animated_png::{AnimatedPngBuilder, AnimatedPngProperties};
pub use animation::{animate_image, animate_image_from_bytes};
pub use auxiliary_data::{AuxiliaryDataInfo, AuxiliaryDataType};
pub use destination::ImageDestination;
pub use error::ImageError;
pub use heif::{HeifBuilder, HeifProperties};
pub use image::{
    convert_format, copy_image_source, decode_bgra, decode_bgra_from_bytes, encode_bgra_to_bytes,
    read_metadata, DecodedImage, ImageFormat, ImageMetadata,
};
pub use metadata::{
    Metadata, MetadataEnumerateOptions, MetadataTag, MetadataType, MutableMetadata,
};
pub use properties::{ImageProperties, MutableProperties};
pub use proraw::{ProRawBuilder, ProRawProperties};
pub use source::{ImageSource, SourceStatus};
pub use thumbnail::{create_thumbnail, ThumbnailOptions};

/// Common imports.
pub mod prelude {
    pub use crate::animated_png::{AnimatedPngBuilder, AnimatedPngProperties};
    pub use crate::animation::{animate_image, animate_image_from_bytes};
    #[cfg(feature = "async")]
    pub use crate::async_api::{
        IncrementalDecodeStream, IncrementalDecodeUpdate, IncrementalImageDecoder,
    };
    pub use crate::auxiliary_data::{AuxiliaryDataInfo, AuxiliaryDataType};
    pub use crate::color_sync::{self, DecodeRequest, EncodeRequest};
    pub use crate::destination::ImageDestination;
    pub use crate::error::ImageError;
    pub use crate::heif::{HeifBuilder, HeifProperties};
    pub use crate::image::{
        convert_format, copy_image_source, decode_bgra, decode_bgra_from_bytes,
        encode_bgra_to_bytes, read_metadata, DecodedImage, ImageFormat, ImageMetadata,
    };
    pub use crate::metadata::{
        Metadata, MetadataEnumerateOptions, MetadataTag, MetadataType, MutableMetadata,
    };
    pub use crate::properties::{ImageProperties, MutableProperties};
    pub use crate::proraw::{ProRawBuilder, ProRawProperties};
    pub use crate::source::{ImageSource, SourceStatus};
    pub use crate::thumbnail::{create_thumbnail, ThumbnailOptions};
}
