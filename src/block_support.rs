//! Helpers for C/Objective-C block interop.

use objc2::encode::{Encode, Encoding, RefEncode};

/// C99 `_Bool` wrapper for block signatures.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CBool(bool);

impl From<bool> for CBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<CBool> for bool {
    fn from(value: CBool) -> Self {
        value.0
    }
}

unsafe impl Encode for CBool {
    const ENCODING: Encoding = Encoding::Bool;
}

unsafe impl RefEncode for CBool {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
