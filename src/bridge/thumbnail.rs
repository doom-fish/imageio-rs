use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_source_create_thumbnail_bgra_at_index(
        raw: Handle,
        index: usize,
        max_pixel_size: usize,
        always_create: bool,
        transform: bool,
        width_out: *mut usize,
        height_out: *mut usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
}
