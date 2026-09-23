use super::common::Handle;

pub type AnimationCallback =
    unsafe extern "C" fn(usize, usize, usize, Handle, *mut std::ffi::c_void) -> bool;

unsafe extern "C" {
    pub fn imageio_animate_image_at_path(
        path: *const i8,
        user_data: *mut std::ffi::c_void,
        callback: AnimationCallback,
        max_width: usize,
        max_height: usize,
        max_bytes: usize,
        timeout_nanoseconds: u64,
        width_out: *mut usize,
        height_out: *mut usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> i32;
    pub fn imageio_animate_image_data(
        bytes: *const u8,
        length: usize,
        user_data: *mut std::ffi::c_void,
        callback: AnimationCallback,
        max_width: usize,
        max_height: usize,
        max_bytes: usize,
        timeout_nanoseconds: u64,
        width_out: *mut usize,
        height_out: *mut usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> i32;
}
