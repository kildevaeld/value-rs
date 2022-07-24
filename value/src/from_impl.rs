#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

#[cfg(feature = "std")]
use std::{collections::BTreeMap, string::String};

use crate::{Number, Value};

#[cfg(feature = "std")]
mod try_from {

    use crate::{Map, Number, Value};
    pub use std::convert::TryFrom;
    use std::fmt;

    macro_rules! from_impl {
        ($type: ty, $method: ident, $as: ident, $as_mut: ident) => {
            impl TryFrom<Value> for $type {
                type Error = FromValueErr<'static>;
                fn try_from(from: Value) -> Result<Self, Self::Error> {
                    match from.$method() {
                        Ok(s) => Ok(s),
                        Err(err) => Err(FromValueErr::Value(err)),
                    }
                }
            }

            impl<'a> TryFrom<&'a Value> for &'a $type {
                type Error = FromValueErr<'a>;
                fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
                    match from.$as() {
                        Some(s) => Ok(s),
                        None => Err(FromValueErr::Ref(from)),
                    }
                }
            }

            // impl<'a> TryFrom<&'a mut Value> for &'a mut $type {
            //     type Error = FromValueErr<'a>;
            //     fn try_from(from: &'a mut Value) -> Result<Self, Self::Error> {
            //         match from.$as_mut() {
            //             Some(s) => Ok(s),
            //             None => Err(FromValueErr::Ref(from)),
            //         }
            //     }
            // }
        };
    }

    #[derive(Debug, Clone)]
    pub enum FromValueErr<'a> {
        Value(Value),
        Ref(&'a Value),
    }

    impl<'a> FromValueErr<'a> {
        pub fn value(&self) -> &Value {
            match self {
                FromValueErr::Ref(ret) => ret,
                FromValueErr::Value(v) => v,
            }
        }
    }

    impl<'a> fmt::Display for FromValueErr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "from error: {:?}", self.value())
        }
    }

    #[cfg(feature = "std")]
    impl<'a> std::error::Error for FromValueErr<'a> {}

    from_impl!(String, into_string, as_string, as_string_mut);
    from_impl!(Vec<u8>, into_bytes, as_bytes, as_bytes_mut);
    from_impl!(bool, into_bool, as_bool, as_bool_mut);
    from_impl!(Number, into_number, as_number, as_number_mut);
    from_impl!(Map, into_map, as_map, as_map_mut);
    from_impl!(Vec<Value>, into_list, as_list, as_list_mut);

    impl<'a> TryFrom<&'a Value> for &'a str {
        type Error = FromValueErr<'a>;
        fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
            match from.as_string() {
                Some(s) => Ok(s),
                None => Err(FromValueErr::Ref(from)),
            }
        }
    }

    impl<'a> TryFrom<&'a Value> for &'a [u8] {
        type Error = FromValueErr<'a>;
        fn try_from(from: &'a Value) -> Result<Self, Self::Error> {
            match from.as_bytes() {
                Some(s) => Ok(s),
                None => Err(FromValueErr::Ref(from)),
            }
        }
    }
}

#[cfg(feature = "std")]
pub use self::try_from::FromValueErr;

macro_rules! from_impl {
    ($from: ty, $map: ident) => {
        impl From<$from> for Value {
            fn from(from: $from) -> Value {
                Value::$map(from.into())
            }
        }
    };

    ($from: ty) => {
        impl From<$from> for Value {
            fn from(from: $from) -> Value {
                Value::Number(from.into())
            }
        }
    };
}

from_impl!(bool, Bool);
from_impl!(Number, Number);
from_impl!(String, String);
from_impl!(Vec<u8>, Bytes);
from_impl!(Vec<Value>, List);
from_impl!(BTreeMap<String, Value>, Map);

impl From<()> for Value {
    fn from(_: ()) -> Value {
        Value::None
    }
}
impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value {
        Value::String(s.to_string())
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(s: &'a [u8]) -> Value {
        Value::Bytes(s.to_vec())
    }
}

from_impl!(u8);
from_impl!(i8);
from_impl!(u16);
from_impl!(i16);
from_impl!(i32);
from_impl!(u32);
from_impl!(i64);
from_impl!(u64);

impl From<f32> for Value {
    fn from(s: f32) -> Value {
        Value::Number(s.into())
    }
}

impl From<f64> for Value {
    fn from(s: f64) -> Value {
        Value::Number(s.into())
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        self
    }
}
