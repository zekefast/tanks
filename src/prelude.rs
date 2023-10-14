pub(crate) use crate::error::Error;

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) use crate::{
    tank::Tank,
};