use super::common::Handle;

unsafe extern "C" {
    pub fn imageio_mutable_properties_create() -> Handle;
    pub fn imageio_mutable_properties_freeze(raw: Handle) -> Handle;
    pub fn imageio_mutable_properties_set_string(
        raw: Handle,
        key: *const i8,
        value: *const i8,
    );
    pub fn imageio_mutable_properties_set_i64(raw: Handle, key: *const i8, value: i64);
    pub fn imageio_mutable_properties_set_f64(raw: Handle, key: *const i8, value: f64);
    pub fn imageio_mutable_properties_set_bool(raw: Handle, key: *const i8, value: bool);
    pub fn imageio_mutable_properties_set_dictionary(
        raw: Handle,
        key: *const i8,
        nested: Handle,
    );
    pub fn imageio_properties_copy_keys(raw: Handle) -> Handle;
    pub fn imageio_properties_has_key(raw: Handle, key: *const i8) -> bool;
    pub fn imageio_properties_copy_string(raw: Handle, key: *const i8) -> Handle;
    pub fn imageio_properties_get_i64(raw: Handle, key: *const i8, out_value: *mut i64)
        -> bool;
    pub fn imageio_properties_get_f64(raw: Handle, key: *const i8, out_value: *mut f64)
        -> bool;
    pub fn imageio_properties_get_bool(
        raw: Handle,
        key: *const i8,
        out_value: *mut bool,
    ) -> bool;
    pub fn imageio_properties_copy_dictionary(raw: Handle, key: *const i8) -> Handle;
}
