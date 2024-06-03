use crate::sys::imp;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct OsError(imp::OsError);

impl Default for OsError {
    fn default() -> Self {
        Self(imp::OsError::empty())
    }
}

impl OsError {
    pub fn into_inner(self) -> imp::OsError {
        self.0
    }
}

impl std::fmt::Display for OsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for OsError {}

impl From<imp::OsError> for OsError {
    fn from(value: imp::OsError) -> Self {
        Self(value)
    }
}
