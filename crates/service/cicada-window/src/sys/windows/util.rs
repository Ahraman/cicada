use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

use windows::core::PCWSTR;

#[derive(Debug, Clone, Default)]
pub(super) struct WideStr {
    buf: Vec<u16>,
}

impl WideStr {
    pub(super) fn from_str(value: impl AsRef<OsStr>) -> Self {
        Self {
            buf: value.as_ref().encode_wide().chain(once(0)).collect(),
        }
    }

    pub(super) fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.buf.as_ptr())
    }
}

#[inline(always)]
pub(super) const fn loword(value: u32) -> u16 {
    (value & 0xffff) as u16
}

#[inline(always)]
pub(super) const fn hiword(value: u32) -> u16 {
    ((value >> 16) & 0xffff) as u16
}
