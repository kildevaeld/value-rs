#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{collections::BTreeMap, string::String};

use crate::{number::Number, NumberType};

#[cfg(feature = "serde")]
use super::de::DeserializerError;
#[cfg(feature = "serde")]
use serde_lib::de::Deserialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ValueType {
    Bool,
    Number(NumberType),
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

impl ValueType {
    pub fn can_cast(&self, ty: &ValueType) -> bool {
        use ValueType::*;
        if self == ty {
            return true;
        }

        match (*self, *ty) {
            (Number(_), Number(_) | String | Bool | Char) => true,
            #[cfg(feature = "datetime")]
            (DateTime | Date, Date | DateTime | String) => true,
            (Bool, Number(_) | String | Char) => true,
            (Char, Bool | Number(_) | String) => true,
            (Map | List | String, Bool) => true,
            (String, Bytes) => true,
            (Map, List) => true,
            (_, List) => true,
            _ => false,
        }
    }
}

macro_rules! is_method {
    ($check: ident, $ty: expr) => {
        pub fn $check(&self) -> bool {
            use ValueType::*;
            self.ty() == $ty
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
    Number(Number),
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
    pub fn ty(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Number(n) => ValueType::Number(n.ty()),
            Value::Char(_) => ValueType::Char,
            Value::String(_) => ValueType::String,
            Value::None => ValueType::None,
            Value::List(_) => ValueType::List,
            Value::Map(_) => ValueType::Map,
            Value::Bytes(_) => ValueType::Bytes,
            #[cfg(feature = "datetime")]
            Value::Date(_) => ValueType::Date,
            #[cfg(feature = "datetime")]
            Value::DateTime(_) => ValueType::DateTime,
        }
    }

    pub fn is(&self, ty: ValueType) -> bool {
        self.ty() == ty
    }

    pub fn is_number_type(&self, ty: NumberType) -> bool {
        match self {
            Value::Number(n) => n.ty() == ty,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    // is_method!(is_number, Number(_));
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

    as_method!(as_number, Number, Number);
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

    into_method!(into_string, String, String);
    into_method!(into_bytes, Bytes, Vec<u8>);
    into_method!(into_bool, Bool, bool);
    into_method!(into_list, List, Vec<Value>);
    into_method!(into_map, Map, BTreeMap<String, Value>);
    into_method!(into_char, Char, char);
    into_method!(into_number, Number, Number);

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

    pub fn convert(self, ty: ValueType) -> Option<Value> {
        let selftype = self.ty();
        if selftype == ty {
            return Some(self);
        }

        match (self, ty) {
            (Value::Number(n), ValueType::Number(_)) => Some(Value::Number(n)),
            (Value::Number(n), ValueType::String) => Some(Value::String(n.to_string())),
            (Value::Number(n), ValueType::Bool) => Some(Value::Bool(n.as_u8() != 0)),
            (Value::Number(n), ValueType::Char) => Some(Value::Char(n.as_u8() as char)),
            (Value::Bool(b), ValueType::String) => Some(Value::String(format!("{}", b))),
            (Value::Bool(b), ValueType::Number(_)) => Some(Value::Number((b as u8).into())),
            (Value::Bool(b), ValueType::Char) => Some(Value::Char((b as u8).into())),
            (Value::Char(c), ValueType::Number(_)) => Some(Value::Number((c as u8).into())),
            (Value::Char(c), ValueType::String) => Some(Value::String(format!("{}", c))),
            (Value::Char(c), ValueType::Bool) => Some(Value::Bool(c as u8 != 0)),
            (Value::String(s), ValueType::Bool) => Some(Value::Bool(!s.is_empty())),
            (Value::String(s), ValueType::Char) => {
                if s.len() == 1 {
                    Some(Value::Char(s.chars().next().unwrap()))
                } else {
                    None
                }
            }
            (Value::String(s), ValueType::Bytes) => Some(Value::Bytes(s.as_bytes().to_vec())),
            (Value::List(l), ValueType::Bool) => Some(Value::Bool(!l.is_empty())),
            (Value::Map(m), ValueType::Bool) => Some(Value::Bool(!m.is_empty())),
            (Value::Map(m), ValueType::List) => {
                let list = m
                    .into_iter()
                    .map(|(k, v)| Value::List(vec![k.into(), v]))
                    .collect();

                Some(Value::List(list))
            }
            (value, ValueType::List) => Some(Value::List(vec![value])),
            _ => None,
        }
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

macro_rules! from_number_impl {
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

from_number_impl!(u8);
from_number_impl!(i8);
from_number_impl!(u16);
from_number_impl!(i16);
from_number_impl!(i32);
from_number_impl!(u32);
from_number_impl!(i64);
from_number_impl!(u64);

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
