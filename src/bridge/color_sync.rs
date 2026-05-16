use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_properties_copy_profile_name(raw: Handle) -> Handle;
    pub fn imageio_source_copy_profile_name_at_index(
        raw: Handle,
        index: usize,
        error_buffer: *mut i8,
        error_buffer_size: usize,
    ) -> Handle;
}
