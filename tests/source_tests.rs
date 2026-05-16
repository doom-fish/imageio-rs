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
