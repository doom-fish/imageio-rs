//! Decode animated GIF frames with `CGImageAnimation`.

use imageio::animate_image;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gif = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/assets/animated.gif");
    let frame_indices = Rc::new(RefCell::new(Vec::new()));
    let frame_indices_for_block = Rc::clone(&frame_indices);

    animate_image(&gif, move |index, frame| {
        println!(
            "frame {index}: {}x{} ({} bytes)",
            frame.width,
            frame.height,
            frame.bgra.len()
        );
        frame_indices_for_block.borrow_mut().push(index);
        index == 0
    })?;

    assert_eq!(frame_indices.borrow().len(), 2);
    Ok(())
}
