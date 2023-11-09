use crate::error::{Error, Result};
pub use crate::myctypes::*;

pub fn wrap_usize(code: c_int) -> Result<usize> {
    code.try_into().map_err(|_| Error::Raw(code))
}

pub fn wrap_unit(code: c_int) -> Result<()> {
    wrap_usize(code).map(|_| ())
}
