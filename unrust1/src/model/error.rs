use super::store;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    // ---- Modules
    Store(store::Error),

    // ---- Externals (e.g., SQLx)
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region: ---- From DB error to Model error.
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<store::Error> for Error {
    fn from(value: store::Error) -> Self {
        Self::Store(value)
    }
}
// endregion: ---- From DB error to Model error.

// region: ---- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}
// endregion: ---- Error Boilerplate
