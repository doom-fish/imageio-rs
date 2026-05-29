//! Declarative macro for retain/release handle wrapper boilerplate.
//!
//! Many `ImageIO` wrapper types hold a single `Handle` (a `*mut c_void`) to a
//! retained Core Foundation / Objective-C object and hand-roll identical
//! `Clone` (retain) and `Drop` (release) implementations. `imageio_retained!`
//! consolidates that boilerplate into a single audited place.
//!
//! The generated impls preserve the exact behavior of the previous
//! hand-written versions:
//! - `Clone` bumps the retain count via [`crate::bridge::retain`].
//! - `Drop` hands the handle to [`crate::bridge::release`] (which already
//!   null-checks before calling the underlying FFI release).
//!
//! All wrappers use a `raw: Handle` field.

/// Generate `Clone` and/or `Drop` impls for a `raw: Handle` wrapper.
///
/// Variants:
/// - `Clone` + `Drop`: `imageio_retained!(Ty);`
/// - `Drop` only: `imageio_retained!(Ty, drop_only);`
macro_rules! imageio_retained {
    // Handle wrapper: Clone (retain) + Drop (release)
    ($ty:ty $(,)?) => {
        impl Clone for $ty {
            fn clone(&self) -> Self {
                Self {
                    raw: $crate::bridge::retain(self.raw),
                }
            }
        }

        impl Drop for $ty {
            fn drop(&mut self) {
                $crate::bridge::release(self.raw);
            }
        }
    };

    // Handle wrapper: Drop only (release)
    ($ty:ty, drop_only $(,)?) => {
        impl Drop for $ty {
            fn drop(&mut self) {
                $crate::bridge::release(self.raw);
            }
        }
    };
}

pub(crate) use imageio_retained;
