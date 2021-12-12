#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "ordered_float")]
use ordered_float_lib::OrderedFloat;
#[cfg(feature = "std")]
use std::{collections::BTreeMap, string::String};

#[cfg(feature = "serde")]
use super::de::DeserializerError;
#[cfg(feature = "serde")]
use serde_lib::de::Deserialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    String,
    List,
    Map,
    Bytes,
    None,
    #[cfg(feature = "datetime")]
    Date,
    #[cfg(feature = "datetime")]
    DateTime,
}

macro_rules! is_method {
    ($check: ident, $ty: ident) => {
        pub fn $check(&self) -> bool {
            self.ty() == Type::$ty
        }
    };
}

macro_rules! into_method {
    ($into: ident, $ty: ident, $oty: ty) => {
        pub fn $into(self) -> Result<$oty, Value> {
            match self {
                Value::$ty(v) => Ok(v),
                _ => Err(self),
            }
        }
    };
}

macro_rules! as_method {
    ($into: ident, $ty: ident, $oty: ty) => {
        pub fn $into(&self) -> Option<&$oty> {
            match &self {
                Value::$ty(v) => Some(v),
                _ => None,
            }
        }
    };
}

#[cfg_attr(
    not(feature = "ordered_float"),
    derive(Debug, Clone, PartialEq, PartialOrd)
)]
#[cfg_attr(
    feature = "ordered_float",
    derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)
)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    #[cfg(feature = "ordered_float")]
    F32(OrderedFloat<f32>),
    #[cfg(feature = "ordered_float")]
    F64(OrderedFloat<f64>),
    #[cfg(not(feature = "ordered_float"))]
    F32(f32),
    #[cfg(not(feature = "ordered_float"))]
    F64(f64),
    Char(char),
    String(String),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Bytes(Vec<u8>),
    #[cfg(feature = "datetime")]
    Date(chrono::NaiveDate),
    #[cfg(feature = "datetime")]
    DateTime(chrono::NaiveDateTime),
    None,
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::U8(_) => Type::U8,
            Value::U16(_) => Type::U16,
            Value::U32(_) => Type::U32,
            Value::U64(_) => Type::U64,
            Value::I8(_) => Type::I8,
            Value::I16(_) => Type::I16,
            Value::I32(_) => Type::I32,
            Value::I64(_) => Type::I64,
            Value::F32(_) => Type::F32,
            Value::F64(_) => Type::F64,
            Value::Char(_) => Type::Char,
            Value::String(_) => Type::String,
            Value::None => Type::None,
            Value::List(_) => Type::List,
            Value::Map(_) => Type::Map,
            Value::Bytes(_) => Type::Bytes,
            #[cfg(feature = "datetime")]
            Value::Date(_) => Type::Date,
            #[cfg(feature = "datetime")]
            Value::DateTime(_) => Type::DateTime,
        }
    }

    pub fn is(&self, ty: Type) -> bool {
        self.ty() == ty
    }

    is_method!(is_i8, I8);
    is_method!(is_u8, U8);
    is_method!(is_i16, I16);
    is_method!(is_u16, U16);
    is_method!(is_i32, I32);
    is_method!(is_u32, U32);
    is_method!(is_u64, U64);
    is_method!(is_i64, I64);
    is_method!(is_string, String);
    is_method!(is_bytes, Bytes);
    is_method!(is_bool, Bool);
    is_method!(is_list, List);
    is_method!(is_map, Map);
    is_method!(is_char, Char);
    is_method!(is_none, None);

    #[cfg(feature = "datetime")]
    is_method!(is_date, Date);
    #[cfg(feature = "datetime")]
    is_method!(is_datetime, DateTime);

    as_method!(as_i8, I8, i8);
    as_method!(as_u8, U8, u8);
    as_method!(as_i16, I16, i16);
    as_method!(as_u16, U16, u16);
    as_method!(as_i32, I32, i32);
    as_method!(as_u32, U32, u32);
    as_method!(as_u64, U64, u64);
    as_method!(as_i64, I64, i64);
    as_method!(as_string, String, String);
    as_method!(as_bytes, Bytes, Vec<u8>);
    as_method!(as_bool, Bool, bool);
    as_method!(as_list, List, Vec<Value>);
    as_method!(as_map, Map, BTreeMap<String, Value>);
    as_method!(as_char, Char, char);

    #[cfg(feature = "datetime")]
    as_method!(as_date, Date, chrono::NaiveDate);
    #[cfg(feature = "datetime")]
    as_method!(as_datetime, DateTime, chrono::NaiveDateTime);

    into_method!(into_i8, I8, i8);
    into_method!(into_u8, U8, u8);
    into_method!(into_i16, I16, i16);
    into_method!(into_u16, U16, u16);
    into_method!(into_i32, I32, i32);
    into_method!(into_u32, U32, u32);
    into_method!(into_u64, U64, u64);
    into_method!(into_i64, I64, i64);
    into_method!(into_string, String, String);
    into_method!(into_bytes, Bytes, Vec<u8>);
    into_method!(into_bool, Bool, bool);
    into_method!(into_list, List, Vec<Value>);
    into_method!(into_map, Map, BTreeMap<String, Value>);
    into_method!(into_char, Char, char);

    #[cfg(feature = "datetime")]
    into_method!(into_date, Date, chrono::NaiveDate);
    #[cfg(feature = "datetime")]
    into_method!(into_datetime, DateTime, chrono::NaiveDateTime);

    pub fn into_option(self) -> Option<Value> {
        match self {
            Value::None => None,
            _ => Some(self),
        }
    }

    #[cfg(feature = "serde")]
    pub fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T, DeserializerError> {
        T::deserialize(self)
    }
}

macro_rules! from_impl {
    ($from: ty, $map: ident) => {
        impl From<$from> for Value {
            fn from(from: $from) -> Value {
                Value::$map(from)
            }
        }
    };
}

from_impl!(bool, Bool);
from_impl!(u8, U8);
from_impl!(i8, I8);
from_impl!(u16, U16);
from_impl!(i16, I16);
from_impl!(i32, I32);
from_impl!(u32, U32);
from_impl!(i64, I64);
from_impl!(u64, U64);
from_impl!(String, String);
from_impl!(Vec<u8>, Bytes);

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value {
        Value::String(s.to_string())
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(s: &'a [u8]) -> Value {
        Value::Bytes(s.to_owned())
    }
}

impl From<f32> for Value {
    fn from(s: f32) -> Value {
        Value::F32(s.into())
    }
}

impl From<f64> for Value {
    fn from(s: f64) -> Value {
        Value::F64(s.into())
    }
}
