pub use std::convert::TryFrom;
use std::{collections::BTreeMap, fmt};

use crate::{number, Number, Typed, Value, ValueType};

#[derive(Debug, Clone)]
pub struct ConvertError {
    expected: ValueType,
    found: ValueType,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Convert error")
    }
}

impl std::error::Error for ConvertError {}

pub trait TryAsRef<S> {
    type Error;
    fn try_as_ref(&self) -> Result<&S, Self::Error>;
}

macro_rules! from_impl {
    ($type: ty, $method: ident) => {
        impl TryFrom<Value> for $type {
            type Error = ConvertError;
            fn try_from(from: Value) -> Result<Self, Self::Error> {
                match from.$method() {
                    Ok(s) => Ok(s),
                    Err(err) => Err(ConvertError {
                        expected: <$type as Typed>::typed(),
                        found: err.ty(),
                    }),
                }
            }
        }
    };
}

macro_rules! try_as_ref {
    ($type: ty, $method: ident) => {
        impl TryAsRef<$type> for Value {
            type Error = ConvertError;
            fn try_as_ref(&self) -> Result<&$type, Self::Error> {
                match self.$method() {
                    Some(s) => Ok(s),
                    None => Err(ConvertError {
                        expected: <$type as Typed>::typed(),
                        found: self.ty(),
                    }),
                }
            }
        }
    };
}

macro_rules! both_impl {
    ($type: ty, $from: ident, $as: ident) => {
        from_impl!($type, $from);
        try_as_ref!($type, $as);
    };
}

both_impl!(String, into_string, as_string);
both_impl!(Vec<u8>, into_bytes, as_bytes);
// both_impl!(Number, into_number, as_number);
both_impl!(BTreeMap<String, Value>, into_map, as_map);
both_impl!(Vec<Value>, into_list, as_list);
