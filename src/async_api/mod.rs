//! Async helpers for progressive `ImageIO` workflows.
//!
//! Gated behind the **`async`** cargo feature.
//!
//! [`IncrementalImageDecoder`] wraps `CGImageSourceCreateIncremental` +
//! `CGImageSourceUpdateData` and publishes executor-agnostic progress updates
//! through [`IncrementalDecodeStream`]. Each update snapshots the current source
//! status, frame status, and any thumbnail that `ImageIO` can synthesize at that
//! point.
//!
//! ## Example
//!
//! ```rust,no_run
//! use imageio::{ThumbnailOptions, async_api::IncrementalImageDecoder};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let png_bytes = std::fs::read("sample.png")?;
//!     let total_chunks = (png_bytes.len() + 255) / 256;
//!     let (mut decoder, updates) = IncrementalImageDecoder::new(0, ThumbnailOptions::new(64))?;
//!
//!     for (index, chunk) in png_bytes.chunks(256).enumerate() {
//!         decoder.update_data(chunk, index + 1 == total_chunks)?;
//!     }
//!
//!     let _source = decoder.into_source();
//!     pollster::block_on(async {
//!         while let Some(update) = updates.next().await {
//!             println!(
//!                 "status={:?} frame={:?} thumbnail={}",
//!                 update.source_status,
//!                 update.frame_status,
//!                 update.thumbnail.is_some()
//!             );
//!         }
//!     });
//!     Ok(())
//! }
//! ```

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::source::{ImageSource, SourceStatus};
use crate::thumbnail::{create_thumbnail, ThumbnailOptions};

const DEFAULT_STREAM_CAPACITY: usize = 8;

/// Snapshot emitted after each incremental data update.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct IncrementalDecodeUpdate {
    /// Frame index tracked by the decoder.
    pub frame_index: usize,
    /// Current overall source status.
    pub source_status: SourceStatus,
    /// Current per-frame status when the frame is visible to `ImageIO`.
    pub frame_status: Option<SourceStatus>,
    /// Current frame count reported by `ImageIO`.
    pub frame_count: usize,
    /// Progressive thumbnail snapshot, when available.
    pub thumbnail: Option<DecodedImage>,
    /// Whether the update that produced this snapshot marked the source final.
    pub is_final: bool,
}

/// Future returned by [`IncrementalDecodeStream::next`].
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct NextUpdate<'a> {
    inner: NextItem<'a, IncrementalDecodeUpdate>,
}

impl Future for NextUpdate<'_> {
    type Output = Option<IncrementalDecodeUpdate>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}

/// Executor-agnostic stream of incremental decode updates.
#[derive(Debug)]
pub struct IncrementalDecodeStream {
    inner: BoundedAsyncStream<IncrementalDecodeUpdate>,
}

impl IncrementalDecodeStream {
    /// Wait for the next incremental decode update.
    pub const fn next(&self) -> NextUpdate<'_> {
        NextUpdate {
            inner: self.inner.next(),
        }
    }

    /// Non-blocking pop of the next buffered update.
    #[must_use]
    pub fn try_next(&self) -> Option<IncrementalDecodeUpdate> {
        self.inner.try_next()
    }

    /// Returns the number of buffered updates.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Returns the configured stream capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns `true` once the decoder has been dropped and all buffered updates are drained.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Drops buffered updates without closing the stream.
    pub fn clear_buffer(&self) {
        self.inner.clear_buffer();
    }
}

/// Progressive incremental decoder backed by `CGImageSourceCreateIncremental`.
#[derive(Debug)]
pub struct IncrementalImageDecoder {
    source: ImageSource,
    frame_index: usize,
    thumbnail_options: ThumbnailOptions,
    buffer: Vec<u8>,
    updates: AsyncStreamSender<IncrementalDecodeUpdate>,
}

impl IncrementalImageDecoder {
    /// Create a decoder and update stream for `frame_index`.
    pub fn new(
        frame_index: usize,
        thumbnail_options: ThumbnailOptions,
    ) -> Result<(Self, IncrementalDecodeStream), ImageError> {
        Self::with_stream_capacity(frame_index, thumbnail_options, DEFAULT_STREAM_CAPACITY)
    }

    /// Create a decoder with an explicit bounded update-stream capacity.
    pub fn with_stream_capacity(
        frame_index: usize,
        thumbnail_options: ThumbnailOptions,
        stream_capacity: usize,
    ) -> Result<(Self, IncrementalDecodeStream), ImageError> {
        let source = ImageSource::incremental()?;
        let (stream, updates) = BoundedAsyncStream::new(stream_capacity);
        Ok((
            Self {
                source,
                frame_index,
                thumbnail_options,
                buffer: Vec::new(),
                updates,
            },
            IncrementalDecodeStream { inner: stream },
        ))
    }

    /// Borrow the underlying incremental image source.
    #[must_use]
    pub const fn source(&self) -> &ImageSource {
        &self.source
    }

    /// Return the tracked frame index.
    #[must_use]
    pub const fn frame_index(&self) -> usize {
        self.frame_index
    }

    /// Return the thumbnail options used for progressive snapshots.
    #[must_use]
    pub const fn thumbnail_options(&self) -> ThumbnailOptions {
        self.thumbnail_options
    }

    /// Append more bytes to the incremental source and emit a progress snapshot.
    pub fn update_data(
        &mut self,
        data: &[u8],
        is_final: bool,
    ) -> Result<IncrementalDecodeUpdate, ImageError> {
        self.buffer.extend_from_slice(data);
        self.source.update_data(&self.buffer, is_final)?;
        let update = self.snapshot(is_final);
        self.updates.push(update.clone());
        Ok(update)
    }

    /// Consume the decoder and return the underlying source.
    #[must_use]
    pub fn into_source(self) -> ImageSource {
        self.source
    }

    fn snapshot(&self, is_final: bool) -> IncrementalDecodeUpdate {
        let frame_count = self.source.frame_count();
        let frame_status =
            (self.frame_index < frame_count).then(|| self.source.status_at_index(self.frame_index));
        let thumbnail = frame_status.and_then(|_| {
            create_thumbnail(&self.source, self.frame_index, self.thumbnail_options).ok()
        });

        IncrementalDecodeUpdate {
            frame_index: self.frame_index,
            source_status: self.source.status(),
            frame_status,
            frame_count,
            thumbnail,
            is_final,
        }
    }
}
