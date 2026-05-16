use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_source_copy_type_identifiers() -> Handle;
    pub fn imageio_source_create_from_path(
        path: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_source_create_from_bytes(
        bytes: *const u8,
        length: usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_source_create_incremental(
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_source_copy_type(raw: Handle) -> Handle;
    pub fn imageio_source_get_count(raw: Handle) -> usize;
    pub fn imageio_source_get_status(raw: Handle) -> i32;
    pub fn imageio_source_get_status_at_index(raw: Handle, index: usize) -> i32;
    pub fn imageio_source_update_data(
        raw: Handle,
        bytes: *const u8,
        length: usize,
        is_final: bool,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_source_copy_properties(
        raw: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_source_copy_properties_at_index(
        raw: Handle,
        index: usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_source_copy_metadata_at_index(raw: Handle, index: usize) -> Handle;
    pub fn imageio_source_copy_auxiliary_data_at_index(
        raw: Handle,
        index: usize,
        auxiliary_type: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_source_get_primary_image_index(raw: Handle) -> usize;
    pub fn imageio_source_remove_cache_at_index(raw: Handle, index: usize);
    pub fn imageio_source_create_bgra_at_index(
        raw: Handle,
        index: usize,
        width_out: *mut usize,
        height_out: *mut usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
}
