#[path = "common/mod.rs"]
mod common;

use imageio::{async_api::IncrementalImageDecoder, prelude::*, ThumbnailOptions};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = common::output_dir("11_async_incremental_decoder")?;
    let png_bytes = common::sample_png_bytes()?;
    let input = dir.join("sample.png");
    fs::write(&input, &png_bytes)?;

    let chunk_len = (png_bytes.len() / 3).max(1);
    let total_chunks = png_bytes.len().div_ceil(chunk_len);
    let (mut decoder, updates) = IncrementalImageDecoder::new(0, ThumbnailOptions::new(1))?;

    for (index, chunk) in png_bytes.chunks(chunk_len).enumerate() {
        let update = decoder.update_data(chunk, index + 1 == total_chunks)?;
        println!(
            "chunk={} status={:?} frame={:?} thumbnail={}",
            index,
            update.source_status,
            update.frame_status,
            update.thumbnail.is_some()
        );
    }

    let source = decoder.into_source();
    let mut last_thumbnail = None;
    pollster::block_on(async {
        while let Some(update) = updates.next().await {
            if let Some(thumbnail) = update.thumbnail {
                last_thumbnail = Some(thumbnail);
            }
        }
    });

    if let Some(thumbnail) = last_thumbnail {
        let thumb_bytes = encode_bgra_to_bytes(
            &thumbnail.bgra,
            thumbnail.width,
            thumbnail.height,
            ImageFormat::Png,
        )?;
        fs::write(dir.join("thumbnail.png"), thumb_bytes)?;
        println!("thumbnail={}x{}", thumbnail.width, thumbnail.height);
    }

    let final_image = source.decode_image_at_index(0)?;
    let decoded_bytes = encode_bgra_to_bytes(
        &final_image.bgra,
        final_image.width,
        final_image.height,
        ImageFormat::Png,
    )?;
    fs::write(dir.join("decoded.png"), decoded_bytes)?;
    println!("decoded={}x{}", final_image.width, final_image.height);

    Ok(())
}
