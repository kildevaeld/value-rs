use core::fmt;
pub use std::convert::TryFrom;

use crate::{Map, Number, Typed, Value, ValueType};

#[derive(Debug, Clone)]
pub struct ConvertError {
    expected: ValueType,
    found: ValueType,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Convert error. Expected {:?}. Found: {:?}",
            self.expected, self.found
        )
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
    ($type: ty, $method: ident, $method_mut: ident) => {
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

        impl<'a> TryFrom<&'a Value> for &'a $type {
            type Error = ConvertError;
            fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
                match from.$method() {
                    Some(s) => Ok(s),
                    None => Err(ConvertError {
                        expected: <$type as Typed>::typed(),
                        found: from.ty(),
                    }),
                }
            }
        }

        impl<'a> TryFrom<&'a mut Value> for &'a mut $type {
            type Error = ConvertError;
            fn try_from(from: &'a mut Value) -> Result<Self, Self::Error> {
                let found = from.ty();
                match from.$method_mut() {
                    Some(s) => Ok(s),
                    None => Err(ConvertError {
                        expected: <$type as Typed>::typed(),
                        found,
                    }),
                }
            }
        }
    };
}

macro_rules! both_impl {
    ($type: ty, $from: ident, $as: ident, $as_mut: ident) => {
        from_impl!($type, $from);
        try_as_ref!($type, $as, $as_mut);
    };
}

both_impl!(String, into_string, as_string, as_string_mut);
both_impl!(Vec<u8>, into_bytes, as_bytes, as_bytes_mut);
both_impl!(bool, into_bool, as_bool, as_bool_mut);
// both_impl!(Number, into_number, as_number);
both_impl!(Map, into_map, as_map, as_map_mut);
both_impl!(Vec<Value>, into_list, as_list, as_list_mut);

impl<'a> TryFrom<&'a Value> for &'a str {
    type Error = ConvertError;
    fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
        match from.as_string() {
            Some(s) => Ok(s),
            None => Err(ConvertError {
                expected: <Self as Typed>::typed(),
                found: from.ty(),
            }),
        }
    }
}
