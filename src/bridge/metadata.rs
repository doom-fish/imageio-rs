use super::common::Handle;

pub type MetadataEnumerateCallback =
    unsafe extern "C" fn(Handle, Handle, *mut std::ffi::c_void) -> bool;

unsafe extern "C" {
    pub fn imageio_metadata_create_from_xmp_data(
        bytes: *const u8,
        length: usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_metadata_create_xmp_data(
        raw: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_mutable_metadata_create(
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_mutable_metadata_create_copy(
        raw: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_mutable_metadata_into_immutable(
        raw: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_metadata_copy_tags(raw: Handle) -> Handle;
    pub fn imageio_metadata_tag_array_count(raw: Handle) -> usize;
    pub fn imageio_metadata_tag_array_copy_item(raw: Handle, index: usize) -> Handle;
    pub fn imageio_metadata_copy_tag_with_path(
        raw: Handle,
        path: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_metadata_copy_string_value_with_path(
        raw: Handle,
        path: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_metadata_enumerate_tags(
        raw: Handle,
        root_path: *const i8,
        recursive: bool,
        user_data: *mut std::ffi::c_void,
        callback: MetadataEnumerateCallback,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_metadata_error_domain() -> Handle;
    pub fn imageio_metadata_register_namespace_for_prefix(
        raw: Handle,
        xmlns: *const i8,
        prefix: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_metadata_set_tag_with_path(
        raw: Handle,
        path: *const i8,
        tag: Handle,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_metadata_set_string_value_with_path(
        raw: Handle,
        path: *const i8,
        value: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_metadata_remove_tag_with_path(
        raw: Handle,
        path: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> bool;
    pub fn imageio_metadata_tag_create_string(
        xmlns: *const i8,
        prefix: *const i8,
        name: *const i8,
        value: *const i8,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
    pub fn imageio_metadata_tag_copy_namespace(raw: Handle) -> Handle;
    pub fn imageio_metadata_tag_copy_prefix(raw: Handle) -> Handle;
    pub fn imageio_metadata_tag_copy_name(raw: Handle) -> Handle;
    pub fn imageio_metadata_tag_copy_string_value(raw: Handle) -> Handle;
    pub fn imageio_metadata_tag_get_type(raw: Handle) -> i32;
    pub fn imageio_metadata_tag_copy_qualifiers(raw: Handle) -> Handle;
}
