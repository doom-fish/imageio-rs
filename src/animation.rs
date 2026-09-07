//! Timed animation helpers backed by `CGImageAnimation`.

use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;

use doom_fish_utils::panic_safe::log_callback_panic;

use crate::bridge::{self, animated_png as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;

struct AnimationState<F> {
    callback: F,
    callback_panicked: bool,
}

unsafe extern "C" fn animation_trampoline<F>(
    index: usize,
    width: usize,
    height: usize,
    data: Handle,
    user_data: *mut c_void,
) -> bool
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let state = unsafe { &mut *user_data.cast::<AnimationState<F>>() };
    match catch_unwind(AssertUnwindSafe(|| {
        (state.callback)(
            index,
            DecodedImage {
                width,
                height,
                bgra: bridge::copy_data(data),
            },
        )
    })) {
        Ok(keep_going) => keep_going,
        Err(payload) => {
            state.callback_panicked = true;
            log_callback_panic("animation_trampoline", payload.as_ref());
            false
        }
    }
}

/// Runs `CGAnimateImageAtURLWithBlock` until finite playback ends or the callback returns `false`.
///
/// Native callbacks are delivered on the main queue, so this synchronous
/// wrapper must be invoked on the process main thread.
pub fn animate_image<F>(path: impl AsRef<Path>, callback: F) -> Result<(), ImageError>
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let path = bridge::path_to_cstring(path.as_ref())?;
    let mut state = AnimationState {
        callback,
        callback_panicked: false,
    };
    let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_animate_image_at_path(
            path.as_ptr(),
            std::ptr::addr_of_mut!(state).cast::<c_void>(),
            animation_trampoline::<F>,
            buffer,
            size,
        )
    });
    if state.callback_panicked {
        return Err(ImageError::DecodeFailed(
            "animation callback panicked".into(),
        ));
    }
    if ok {
        Ok(())
    } else {
        Err(ImageError::DecodeFailed(if message.is_empty() {
            "imageio_animate_image_at_path returned false".into()
        } else {
            message
        }))
    }
}

/// Runs `CGAnimateImageDataWithBlock` until finite playback ends or the callback returns `false`.
///
/// Native callbacks are delivered on the main queue, so this synchronous
/// wrapper must be invoked on the process main thread.
pub fn animate_image_from_bytes<F>(data: &[u8], callback: F) -> Result<(), ImageError>
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let mut state = AnimationState {
        callback,
        callback_panicked: false,
    };
    let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_animate_image_data(
            data.as_ptr(),
            data.len(),
            std::ptr::addr_of_mut!(state).cast::<c_void>(),
            animation_trampoline::<F>,
            buffer,
            size,
        )
    });
    if state.callback_panicked {
        return Err(ImageError::DecodeFailed(
            "animation callback panicked".into(),
        ));
    }
    if ok {
        Ok(())
    } else {
        Err(ImageError::DecodeFailed(if message.is_empty() {
            "imageio_animate_image_data returned false".into()
        } else {
            message
        }))
    }
}
