use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_destination_copy_type_identifiers() -> Handle;
    pub fn imageio_destination_create_with_url(
        path: *const i8,
        type_identifier: *const i8,
        image_count: usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_destination_create_with_data(
        type_identifier: *const i8,
        image_count: usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_destination_set_properties(raw: Handle, properties: Handle);
    pub fn imageio_destination_add_bgra_image(
        raw: Handle,
        bytes: *const u8,
        length: usize,
        width: usize,
        height: usize,
        properties: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_add_bgra_image_with_metadata(
        raw: Handle,
        bytes: *const u8,
        length: usize,
        width: usize,
        height: usize,
        metadata: Handle,
        properties: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_add_cg_image(
        raw: Handle,
        cg_image: super::super::ffi::CGImageRef,
        properties: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_add_image_from_source(
        raw: Handle,
        source: Handle,
        index: usize,
        properties: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_copy_image_source(
        raw: Handle,
        source: Handle,
        properties: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_add_auxiliary_data_info(
        raw: Handle,
        auxiliary_type: *const i8,
        info: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_finalize(
        raw: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_destination_copy_data(raw: Handle) -> Handle;
}
