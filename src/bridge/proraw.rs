use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_proraw_copy_raw_dictionary(raw: Handle) -> Handle;
    pub fn imageio_proraw_copy_dng_dictionary(raw: Handle) -> Handle;
    pub fn imageio_proraw_copy_profile_name(raw: Handle) -> Handle;
    pub fn imageio_proraw_copy_unique_camera_model(raw: Handle) -> Handle;
}
