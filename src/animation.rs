//! Frame-by-frame animation helpers built on `ImageIO` sources.

use std::ffi::c_void;
use std::path::Path;

use doom_fish_utils::panic_safe::catch_user_panic;

use crate::bridge::{self, animated_png as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;

struct AnimationState<F> {
    callback: F,
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
    let mut result = false;
    catch_user_panic("animation_trampoline", || {
        result = (state.callback)(
            index,
            DecodedImage {
                width,
                height,
                bgra: bridge::copy_data(data),
            },
        );
    });
    result
}

pub fn animate_image<F>(path: impl AsRef<Path>, callback: F) -> Result<(), ImageError>
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let path = bridge::path_to_cstring(path.as_ref())?;
    let mut state = AnimationState { callback };
    let (ok, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_animate_image_at_path(
            path.as_ptr(),
            std::ptr::addr_of_mut!(state).cast::<c_void>(),
            animation_trampoline::<F>,
            buffer,
            size,
        )
    });
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

pub fn animate_image_from_bytes<F>(data: &[u8], callback: F) -> Result<(), ImageError>
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let mut state = AnimationState { callback };
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
