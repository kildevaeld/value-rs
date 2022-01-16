use core::fmt;

use alloc::vec::Vec;
use value::ValueType;

#[derive(Debug)]
pub enum Error {
    Multi(Vec<Error>),
    Required,
    InvalidType {
        expected: ValueType,
        found: ValueType,
    },
    Min {
        min: usize,
    },
    OneOf(Vec<Error>),
    Equal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
