use std::ffi::{c_char, c_void, CStr, CString};
use std::path::Path;

use crate::error::ImageError;

pub type Handle = *mut c_void;
const ERROR_BUFFER_SIZE: usize = 1024;

unsafe extern "C" {
    pub fn imageio_retain(raw: Handle) -> Handle;
    pub fn imageio_release(raw: Handle);
    fn imageio_string_len(raw: Handle) -> usize;
    fn imageio_string_copy_utf8(raw: Handle, buffer: *mut u8, capacity: usize) -> usize;
    fn imageio_data_len(raw: Handle) -> usize;
    fn imageio_data_copy_bytes(raw: Handle, buffer: *mut u8, capacity: usize) -> usize;
    fn imageio_string_array_count(raw: Handle) -> usize;
    fn imageio_string_array_copy_item(raw: Handle, index: usize) -> Handle;
}

pub fn retain(raw: Handle) -> Handle {
    unsafe { imageio_retain(raw) }
}

pub fn release(raw: Handle) {
    if !raw.is_null() {
        unsafe { imageio_release(raw) };
    }
}

pub fn copy_string(raw: Handle) -> Option<String> {
    if raw.is_null() {
        return None;
    }
    let len = unsafe { imageio_string_len(raw) };
    let mut buffer = vec![0_u8; len.saturating_add(1)];
    unsafe {
        imageio_string_copy_utf8(raw, buffer.as_mut_ptr(), buffer.len());
        imageio_release(raw);
    }
    let end = buffer
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(buffer.len());
    buffer.truncate(end);
    String::from_utf8(buffer).ok()
}

pub fn copy_data(raw: Handle) -> Vec<u8> {
    if raw.is_null() {
        return Vec::new();
    }
    let len = unsafe { imageio_data_len(raw) };
    let mut buffer = vec![0_u8; len];
    if len > 0 {
        unsafe { imageio_data_copy_bytes(raw, buffer.as_mut_ptr(), buffer.len()) };
    }
    unsafe { imageio_release(raw) };
    buffer
}

pub fn copy_string_array(raw: Handle) -> Vec<String> {
    if raw.is_null() {
        return Vec::new();
    }
    let count = unsafe { imageio_string_array_count(raw) };
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        if let Some(value) = copy_string(unsafe { imageio_string_array_copy_item(raw, index) }) {
            values.push(value);
        }
    }
    release(raw);
    values
}

pub fn cstring(value: &str) -> Result<CString, ImageError> {
    CString::new(value)
        .map_err(|err| ImageError::Unknown(format!("CString conversion failed: {err}")))
}

pub fn path_to_cstring(path: &Path) -> Result<CString, ImageError> {
    let path = path
        .to_str()
        .ok_or_else(|| ImageError::InvalidPath("non-UTF-8 path".into()))?;
    cstring(path)
}

pub fn with_error_buffer<F, R>(f: F) -> (R, String)
where
    F: FnOnce(*mut c_char, usize) -> R,
{
    let mut buffer = vec![0_i8; ERROR_BUFFER_SIZE];
    let result = f(buffer.as_mut_ptr(), buffer.len());
    let message = unsafe { CStr::from_ptr(buffer.as_ptr()) }
        .to_string_lossy()
        .trim_end_matches('\0')
        .to_string();
    (result, message)
}
