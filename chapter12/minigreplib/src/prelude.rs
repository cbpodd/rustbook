// prelude.rs

pub(crate) use crate::error::Error;
pub(crate) use crate::newtypes::*;

pub(crate) type Result<T> = core::result::Result<T, Error>;
