mod common;

use imageio::prelude::*;

#[test]
fn source_reads_png_and_decodes_pixels() {
    let dir = common::work_dir("source_reads_png_and_decodes_pixels");
    let png = dir.join("sample.png");
    common::write_sample_png(&png);

    let source = ImageSource::from_path(&png).expect("open sample png");
    let decoded = source.decode_image_at_index(0).expect("decode sample png");

    assert_eq!(source.frame_count(), 1);
    assert_eq!(decoded.width, common::sample_image().width);
    assert_eq!(decoded.height, common::sample_image().height);
    assert_eq!(source.status(), SourceStatus::Complete);
}

#[test]
fn source_options_reach_every_constructor() {
    let png = common::sample_png_bytes();
    let expected = common::sample_image();
    let path = common::work_dir("source_options").join("sample.png");
    common::write_sample_png(&path);

    for should_cache in [None, Some(false), Some(true)] {
        let mut options = ImageSourceOptions::default();
        options.type_identifier_hint = Some(ImageFormat::Png.type_identifier().to_owned());
        options.should_cache = should_cache;

        let sources = [
            ImageSource::from_bytes_with_options(&png, &options).expect("bytes source"),
            ImageSource::from_path_with_options(&path, &options).expect("path source"),
            ImageSource::from_data_provider(
                &DataProvider::from_bytes(&png).expect("bytes provider"),
                &options,
            )
            .expect("bytes provider source"),
            ImageSource::from_data_provider(
                &DataProvider::from_path(&path).expect("path provider"),
                &options,
            )
            .expect("path provider source"),
        ];
        for source in &sources {
            assert_eq!(source.source_type().as_deref(), Some("public.png"));
            assert_eq!(source.decode_image_at_index(0).as_ref(), Ok(&expected));
        }
    }
}

#[test]
fn incremental_sources_accept_data_providers() {
    let png = common::sample_png_bytes();
    let mut options = ImageSourceOptions::default();
    options.should_cache = Some(false);
    let mut partial = ImageSource::incremental_with_options(&options).expect("incremental source");
    assert_eq!(partial.frame_count(), 0);
    partial
        .update_data_provider(
            &DataProvider::from_bytes(&png[..8]).expect("partial"),
            false,
        )
        .expect("partial update");
    assert_ne!(partial.status(), SourceStatus::Complete);

    let mut complete = ImageSource::incremental_with_options(&options).expect("incremental source");
    complete
        .update_data_provider(&DataProvider::from_bytes(&png).expect("complete"), true)
        .expect("final update");
    assert_eq!(complete.status(), SourceStatus::Complete);
    assert_eq!(
        complete.decode_image_at_index(0),
        Ok(common::sample_image())
    );
}

#[test]
fn source_options_reject_interior_nul_in_the_type_hint() {
    let mut options = ImageSourceOptions::default();
    options.type_identifier_hint = Some("public.png\0".to_owned());
    assert!(matches!(
        ImageSource::from_bytes_with_options(&common::sample_png_bytes(), &options),
        Err(ImageError::Unknown(_))
    ));
    assert!(
        DataProvider::from_path(common::work_dir("source_options").join("missing.png")).is_err()
    );
}
