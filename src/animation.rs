//! Helpers for `CGImageAnimation` frame callbacks.

use core::ptr;
use std::cell::{Cell, RefCell};
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};

use block2::RcBlock;

use crate::block_support::CBool;
use crate::error::ImageError;
use crate::ffi;
use crate::image::DecodedImage;
use crate::util::{cg_image_to_bgra, make_cf_data, make_file_url};

const RUN_LOOP_SLICE_SECS: f64 = 0.05;
const RUN_LOOP_TIMEOUT: Duration = Duration::from_secs(5);

const fn animation_status_message(status: ffi::CGImageAnimationStatus) -> &'static str {
    match status {
        ffi::kCGImageAnimationStatus_ParameterError => "parameter error",
        ffi::kCGImageAnimationStatus_CorruptInputImage => "corrupt input image",
        ffi::kCGImageAnimationStatus_UnsupportedFormat => "unsupported format",
        ffi::kCGImageAnimationStatus_IncompleteInputImage => "incomplete input image",
        ffi::kCGImageAnimationStatus_AllocationFailure => "allocation failure",
        _ => "unknown animation failure",
    }
}

fn pump_run_loop_until(finished: &Cell<bool>) -> Result<(), ImageError> {
    let deadline = Instant::now() + RUN_LOOP_TIMEOUT;
    while !finished.get() {
        if Instant::now() >= deadline {
            return Err(ImageError::Unknown(
                "timed out waiting for CGImageAnimation callback; return false from the callback to stop playback"
                    .into(),
            ));
        }
        unsafe {
            let _ = ffi::CFRunLoopRunInMode(ffi::kCFRunLoopDefaultMode, RUN_LOOP_SLICE_SECS, true);
        }
    }
    Ok(())
}

fn animate_with_status(
    status: ffi::CGImageAnimationStatus,
    callback_error: Option<ImageError>,
) -> Result<(), ImageError> {
    if let Some(error) = callback_error {
        return Err(error);
    }
    if status == 0 {
        Ok(())
    } else {
        Err(ImageError::DecodeFailed(format!(
            "CGImageAnimation failed with status {status}: {}",
            animation_status_message(status)
        )))
    }
}

/// Animate an image file and decode each frame to BGRA.
///
/// Return `false` from the callback to stop playback; the function pumps the
/// main run loop until that happens.
pub fn animate_image(
    path: impl AsRef<Path>,
    callback: impl FnMut(usize, DecodedImage) -> bool + 'static,
) -> Result<(), ImageError> {
    let url = make_file_url(path.as_ref())?;
    let callback = RefCell::new(callback);
    let callback_error = Rc::new(RefCell::new(None));
    let callback_error_for_block = Rc::clone(&callback_error);
    let finished = Rc::new(Cell::new(false));
    let finished_for_block = Rc::clone(&finished);
    let block: RcBlock<dyn Fn(usize, ffi::CGImageRef, *mut CBool)> = RcBlock::new(
        move |index: usize, image: ffi::CGImageRef, stop: *mut CBool| match cg_image_to_bgra(image)
        {
            Ok(frame) => {
                let should_continue = callback.borrow_mut()(index, frame);
                if !should_continue {
                    finished_for_block.set(true);
                    if !stop.is_null() {
                        unsafe { *stop = CBool::from(true) };
                    }
                }
            }
            Err(error) => {
                *callback_error_for_block.borrow_mut() = Some(error);
                finished_for_block.set(true);
                if !stop.is_null() {
                    unsafe { *stop = CBool::from(true) };
                }
            }
        },
    );
    let block_ptr: *const block2::Block<dyn Fn(usize, ffi::CGImageRef, *mut CBool)> = &*block;
    let status = unsafe { ffi::CGAnimateImageAtURLWithBlock(url, ptr::null(), block_ptr.cast()) };
    unsafe { ffi::CFRelease(url.cast()) };
    animate_with_status(status, callback_error.borrow_mut().take())?;
    pump_run_loop_until(&finished)
}

/// Animate encoded image bytes and decode each frame to BGRA.
///
/// Return `false` from the callback to stop playback; the function pumps the
/// main run loop until that happens.
pub fn animate_image_from_bytes(
    data: &[u8],
    callback: impl FnMut(usize, DecodedImage) -> bool + 'static,
) -> Result<(), ImageError> {
    let data = make_cf_data(data)?;
    let callback = RefCell::new(callback);
    let callback_error = Rc::new(RefCell::new(None));
    let callback_error_for_block = Rc::clone(&callback_error);
    let finished = Rc::new(Cell::new(false));
    let finished_for_block = Rc::clone(&finished);
    let block: RcBlock<dyn Fn(usize, ffi::CGImageRef, *mut CBool)> = RcBlock::new(
        move |index: usize, image: ffi::CGImageRef, stop: *mut CBool| match cg_image_to_bgra(image)
        {
            Ok(frame) => {
                let should_continue = callback.borrow_mut()(index, frame);
                if !should_continue {
                    finished_for_block.set(true);
                    if !stop.is_null() {
                        unsafe { *stop = CBool::from(true) };
                    }
                }
            }
            Err(error) => {
                *callback_error_for_block.borrow_mut() = Some(error);
                finished_for_block.set(true);
                if !stop.is_null() {
                    unsafe { *stop = CBool::from(true) };
                }
            }
        },
    );
    let block_ptr: *const block2::Block<dyn Fn(usize, ffi::CGImageRef, *mut CBool)> = &*block;
    let status = unsafe { ffi::CGAnimateImageDataWithBlock(data, ptr::null(), block_ptr.cast()) };
    unsafe { ffi::CFRelease(data.cast()) };
    animate_with_status(status, callback_error.borrow_mut().take())?;
    pump_run_loop_until(&finished)
}
