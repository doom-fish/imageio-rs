use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_heif_copy_dictionary(raw: Handle) -> Handle;
    pub fn imageio_heics_copy_dictionary(raw: Handle) -> Handle;
    pub fn imageio_heif_get_primary(raw: Handle, out_value: *mut bool) -> bool;
    pub fn imageio_heics_get_loop_count(raw: Handle, out_value: *mut i64) -> bool;
    pub fn imageio_heics_get_delay_time(raw: Handle, out_value: *mut f64) -> bool;
    pub fn imageio_heics_get_unclamped_delay_time(raw: Handle, out_value: *mut f64) -> bool;
    pub fn imageio_heics_get_canvas_width(raw: Handle, out_value: *mut i64) -> bool;
    pub fn imageio_heics_get_canvas_height(raw: Handle, out_value: *mut i64) -> bool;
    pub fn imageio_heics_get_frame_info_count(raw: Handle) -> usize;
}
