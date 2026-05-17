pub mod animated_png;
pub mod auxiliary_data;
pub mod color_sync;
pub mod common;
pub mod destination;
pub mod metadata;
pub mod properties;
pub mod source;
pub mod thumbnail;

pub use common::{
    copy_data, copy_string, copy_string_array, cstring, path_to_cstring, release, retain,
    with_error_buffer, Handle,
};
