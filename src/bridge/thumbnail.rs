use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_source_create_thumbnail_bgra_at_index(
        raw: Handle,
        index: usize,
        max_pixel_size: usize,
        always_create: bool,
        transform: bool,
        max_width: usize,
        max_height: usize,
        max_bytes: usize,
        width_out: *mut usize,
        height_out: *mut usize,
        limit_exceeded: *mut bool,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
}
