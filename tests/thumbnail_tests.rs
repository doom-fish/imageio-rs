mod common;

use imageio::prelude::*;

#[test]
fn thumbnail_generation_downscales_image() {
    let dir = common::work_dir("thumbnail_generation_downscales_image");
    let png = dir.join("sample.png");
    common::write_sample_png(&png);

    let source = ImageSource::from_path(&png).expect("open sample png");
    let thumbnail = create_thumbnail(&source, 0, ThumbnailOptions::new(1)).expect("create thumbnail");

    assert!(thumbnail.width <= common::sample_image().width);
    assert!(thumbnail.height <= common::sample_image().height);
}
