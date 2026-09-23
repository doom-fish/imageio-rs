//! Timed animation helpers backed by `CGImageAnimation`.

use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::time::Duration;

use doom_fish_utils::panic_safe::log_callback_panic;

use crate::bridge::{self, animated_png as ffi, Handle};
use crate::error::ImageError;
use crate::image::DecodedImage;
use crate::limits::DecodeLimits;

const ANIMATION_SUCCEEDED: i32 = 0;
const ANIMATION_TIMED_OUT: i32 = 2;
const ANIMATION_LIMIT_EXCEEDED: i32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AnimationOptions {
    pub timeout: Option<Duration>,
    pub limits: DecodeLimits,
}

impl AnimationOptions {
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);

    fn timeout_nanoseconds(self) -> u64 {
        self.timeout.map_or(u64::MAX, |timeout| {
            u64::try_from(timeout.as_nanos()).map_or(u64::MAX - 1, |value| value.min(u64::MAX - 1))
        })
    }

    fn result(
        self,
        status: i32,
        width: usize,
        height: usize,
        message: String,
        fallback: &str,
    ) -> Result<(), ImageError> {
        match status {
            ANIMATION_SUCCEEDED => Ok(()),
            ANIMATION_TIMED_OUT => Err(ImageError::Timeout(self.timeout.unwrap_or(Duration::MAX))),
            ANIMATION_LIMIT_EXCEEDED => Err(ImageError::LimitExceeded {
                width,
                height,
                limits: self.limits,
            }),
            _ => Err(ImageError::DecodeFailed(if message.is_empty() {
                fallback.into()
            } else {
                message
            })),
        }
    }
}

impl Default for AnimationOptions {
    fn default() -> Self {
        Self {
            timeout: Some(Self::DEFAULT_TIMEOUT),
            limits: DecodeLimits::DEFAULT,
        }
    }
}

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
/// wrapper must be invoked on the process main thread. It also returns when
/// `ImageIO` ends playback early, and fails with [`ImageError::Timeout`] once
/// `options.timeout` has elapsed; no callback runs after it returns.
pub fn animate_image<F>(
    path: impl AsRef<Path>,
    options: AnimationOptions,
    callback: F,
) -> Result<(), ImageError>
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let path = bridge::path_to_cstring(path.as_ref())?;
    let (max_width, max_height, max_bytes) = options.limits.bridge_values();
    let mut state = AnimationState {
        callback,
        callback_panicked: false,
    };
    let mut width = 0_usize;
    let mut height = 0_usize;
    let (status, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_animate_image_at_path(
            path.as_ptr(),
            std::ptr::addr_of_mut!(state).cast::<c_void>(),
            animation_trampoline::<F>,
            max_width,
            max_height,
            max_bytes,
            options.timeout_nanoseconds(),
            &raw mut width,
            &raw mut height,
            buffer,
            size,
        )
    });
    if state.callback_panicked {
        return Err(ImageError::DecodeFailed(
            "animation callback panicked".into(),
        ));
    }
    options.result(
        status,
        width,
        height,
        message,
        "imageio_animate_image_at_path failed",
    )
}

/// Runs `CGAnimateImageDataWithBlock` until finite playback ends or the callback returns `false`.
///
/// Native callbacks are delivered on the main queue, so this synchronous
/// wrapper must be invoked on the process main thread. It also returns when
/// `ImageIO` ends playback early, and fails with [`ImageError::Timeout`] once
/// `options.timeout` has elapsed; no callback runs after it returns.
pub fn animate_image_from_bytes<F>(
    data: &[u8],
    options: AnimationOptions,
    callback: F,
) -> Result<(), ImageError>
where
    F: FnMut(usize, DecodedImage) -> bool,
{
    let (max_width, max_height, max_bytes) = options.limits.bridge_values();
    let mut state = AnimationState {
        callback,
        callback_panicked: false,
    };
    let mut width = 0_usize;
    let mut height = 0_usize;
    let (status, message) = bridge::with_error_buffer(|buffer, size| unsafe {
        ffi::imageio_animate_image_data(
            data.as_ptr(),
            data.len(),
            std::ptr::addr_of_mut!(state).cast::<c_void>(),
            animation_trampoline::<F>,
            max_width,
            max_height,
            max_bytes,
            options.timeout_nanoseconds(),
            &raw mut width,
            &raw mut height,
            buffer,
            size,
        )
    });
    if state.callback_panicked {
        return Err(ImageError::DecodeFailed(
            "animation callback panicked".into(),
        ));
    }
    options.result(
        status,
        width,
        height,
        message,
        "imageio_animate_image_data failed",
    )
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{
        AnimationOptions, ANIMATION_LIMIT_EXCEEDED, ANIMATION_SUCCEEDED, ANIMATION_TIMED_OUT,
    };
    use crate::error::ImageError;
    use crate::limits::DecodeLimits;

    #[test]
    fn default_options_bound_the_wait_and_the_frame_size() {
        let options = AnimationOptions::default();
        assert_eq!(options.timeout, Some(AnimationOptions::DEFAULT_TIMEOUT));
        assert_eq!(options.limits, DecodeLimits::DEFAULT);
        assert_eq!(options.timeout_nanoseconds(), 60_000_000_000);
    }

    #[test]
    fn timeouts_convert_to_bridge_nanoseconds() {
        let with_timeout = |timeout| AnimationOptions {
            timeout,
            ..AnimationOptions::default()
        };
        assert_eq!(with_timeout(None).timeout_nanoseconds(), u64::MAX);
        assert_eq!(
            with_timeout(Some(Duration::MAX)).timeout_nanoseconds(),
            u64::MAX - 1
        );
        assert_eq!(
            with_timeout(Some(Duration::from_millis(5))).timeout_nanoseconds(),
            5_000_000
        );
    }

    #[test]
    fn bridge_status_maps_to_results() {
        let options = AnimationOptions {
            timeout: Some(Duration::from_millis(250)),
            ..AnimationOptions::default()
        };
        assert_eq!(
            options.result(ANIMATION_SUCCEEDED, 0, 0, String::new(), "fallback"),
            Ok(())
        );
        assert_eq!(
            options.result(ANIMATION_TIMED_OUT, 0, 0, String::new(), "fallback"),
            Err(ImageError::Timeout(Duration::from_millis(250)))
        );
        assert_eq!(
            options.result(ANIMATION_LIMIT_EXCEEDED, 32, 16, String::new(), "fallback"),
            Err(ImageError::LimitExceeded {
                width: 32,
                height: 16,
                limits: DecodeLimits::DEFAULT,
            })
        );
        assert_eq!(
            options.result(1, 0, 0, "boom".into(), "fallback"),
            Err(ImageError::DecodeFailed("boom".into()))
        );
        assert_eq!(
            options.result(1, 0, 0, String::new(), "fallback"),
            Err(ImageError::DecodeFailed("fallback".into()))
        );
    }
}
