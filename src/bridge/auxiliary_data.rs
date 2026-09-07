use super::common::{Handle, NativeHandle};

unsafe extern "C" {
    pub fn imageio_auxiliary_data_info_create() -> Handle;
    pub fn imageio_auxiliary_data_info_set_data(raw: Handle, bytes: *const u8, length: usize);
    pub fn imageio_auxiliary_data_info_set_description(raw: Handle, properties: Handle);
    pub fn imageio_auxiliary_data_info_set_metadata(raw: Handle, metadata: Handle);
    pub fn imageio_auxiliary_data_info_set_color_space(raw: Handle, color_space: NativeHandle);
    pub fn imageio_auxiliary_data_info_copy_data(raw: Handle) -> Handle;
    pub fn imageio_auxiliary_data_info_copy_description(raw: Handle) -> Handle;
    pub fn imageio_auxiliary_data_info_copy_metadata(raw: Handle) -> Handle;
    pub fn imageio_auxiliary_data_info_copy_color_space(raw: Handle) -> NativeHandle;
    pub fn imageio_auxiliary_data_info_has_color_space(raw: Handle) -> bool;
}
