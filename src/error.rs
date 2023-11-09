use crate::ffi::c_int;
use core::fmt;
use core::num::TryFromIntError;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Raw(c_int),
    TryFromInt(TryFromIntError),
    Invalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromInt(e)
    }
}
