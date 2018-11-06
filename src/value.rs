use super::map::Map;
use super::number::Number;
#[cfg(feature = "datetime")]
use chrono;
use std::fmt::{self, Debug};
use std::mem;

#[cfg(feature = "datetime")]
pub type Date = chrono::Date<chrono::Utc>;
#[cfg(feature = "datetime")]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(PartialEq, Clone)]
pub enum Value {
    Null,
    String(String),
    Number(Number),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Object(Map<String, Value>),
    Bool(bool),
    #[cfg(feature = "datetime")]
    Date(Date),
    #[cfg(feature = "datetime")]
    DateTime(DateTime),
}

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => formatter.debug_tuple("Null").finish(),
            Value::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            Value::Number(ref v) => Debug::fmt(v, formatter),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Array(ref v) => formatter.debug_tuple("Array").field(v).finish(),
            Value::Object(ref v) => formatter.debug_tuple("Object").field(v).finish(),
            Value::Bytes(ref v) => formatter.debug_tuple("Bytes").field(v).finish(),
            #[cfg(feature = "datetime")]
            Value::Date(ref v) => formatter.debug_tuple("DateTime").field(v).finish(),
            #[cfg(feature = "datetime")]
            Value::DateTime(ref v) => formatter.debug_tuple("DateTime").field(v).finish(),
        }
    }
}

impl Value {
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Object(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match *self {
            Value::Object(ref mut v) => Some(v),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut v) => Some(v),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_null(&self) -> Option<()> {
        match *self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn as_number(&self) -> Option<&Number> {
        match *self {
            Value::Number(ref n) => Some(n),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        self.as_number().is_some()
    }

    pub fn take(&mut self) -> Value {
        mem::replace(self, Value::Null)
    }

    #[cfg(feature = "datetime")]
    pub fn as_date_time(&self) -> Option<&DateTime> {
        match *self {
            Value::DateTime(ref n) => Some(n),
            _ => None,
        }
    }

    #[cfg(feature = "datetime")]
    pub fn is_date_time(&self) -> bool {
        self.as_date_time().is_some()
    }

    #[cfg(feature = "datetime")]
    pub fn as_date(&self) -> Option<&Date> {
        match *self {
            Value::Date(ref n) => Some(n),
            _ => None,
        }
    }

    #[cfg(feature = "datetime")]
    pub fn is_date(&self) -> bool {
        self.as_date().is_some()
    }
}
