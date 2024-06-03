#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
pub(crate) mod imp;

pub(crate) use imp::*;
