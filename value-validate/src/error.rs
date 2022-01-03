use value::ValueType;

use alloc::vec::Vec;

#[derive(Debug)]
pub enum Error {
    Multi(Vec<Error>),
    InvalidType {
        expected: ValueType,
        found: ValueType,
    },
}
