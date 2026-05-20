#![cfg(feature = "async")]

mod common;

use imageio::{async_api::IncrementalImageDecoder, prelude::*};

#[test]
fn incremental_decoder_emits_updates_until_complete() {
    let bytes = common::sample_png_bytes();
    let split_at = (bytes.len() / 2).max(1);
    let (mut decoder, updates) =
        IncrementalImageDecoder::new(0, ThumbnailOptions::new(1)).expect("create decoder");

    let first = decoder
        .update_data(&bytes[..split_at], false)
        .expect("feed first chunk");
    let second = decoder
        .update_data(&bytes[split_at..], true)
        .expect("feed final chunk");
    let source = decoder.into_source();

    assert!(!first.is_final);
    assert!(matches!(
        first.source_status,
        SourceStatus::ReadingHeader | SourceStatus::Incomplete
    ));
    assert_eq!(second.source_status, SourceStatus::Complete);
    assert_eq!(second.frame_status, Some(SourceStatus::Complete));
    assert!(second.thumbnail.is_some());
    assert_eq!(source.status(), SourceStatus::Complete);

    let drained = pollster::block_on(async {
        let mut items = Vec::new();
        while let Some(update) = updates.next().await {
            items.push(update);
        }
        items
    });

    assert_eq!(drained, vec![first, second]);

    let final_image = source.decode_image_at_index(0).expect("decode final image");
    assert_eq!(final_image.width, common::sample_image().width);
    assert_eq!(final_image.height, common::sample_image().height);
}

#[test]
fn update_stream_closes_when_decoder_drops() {
    let stream = {
        let (_decoder, stream) =
            IncrementalImageDecoder::new(0, ThumbnailOptions::default()).expect("create decoder");
        stream
    };

    let update = pollster::block_on(stream.next());
    assert!(update.is_none());
    assert!(stream.is_closed());
}
