use super::map::Map;
use super::number::Number;
use super::value::Value;

#[cfg(feature = "datetime")]
use super::value::{Date, DateTime};

pub trait ToValue {
    fn to_value(self) -> Value;
}

macro_rules! to_value_impl {
    ($t: ident, $v: ident) => {
        impl ToValue for $t {
            fn to_value(self) -> Value {
                Value::$v(self)
            }
        }
    };
}

to_value_impl!(Number, Number);
to_value_impl!(bool, Bool);
to_value_impl!(String, String);

impl ToValue for &str {
    fn to_value(self) -> Value {
        Value::String(self.to_owned())
    }
}

impl ToValue for Vec<u8> {
    fn to_value(self) -> Value {
        Value::Bytes(self)
    }
}

impl ToValue for &[u8] {
    fn to_value(self) -> Value {
        Value::Bytes(self.to_vec())
    }
}

impl ToValue for Map<String, Value> {
    fn to_value(self) -> Value {
        Value::Object(self)
    }
}

impl ToValue for Vec<Value> {
    fn to_value(self) -> Value {
        Value::Array(self)
    }
}

#[cfg(feature = "datetime")]
impl ToValue for Date {
    fn to_value(self) -> Value {
        Value::Date(self)
    }
}

#[cfg(feature = "datetime")]
impl ToValue for DateTime {
    fn to_value(self) -> Value {
        Value::DateTime(self)
    }
}

impl ToValue for Value {
    fn to_value(self) -> Value {
        self
    }
}

macro_rules! to_number_impl {
    ($ty:ident, $as:ident, $fn: ident) => {
        impl ToValue for $ty {
            fn to_value(self) -> Value {
                Value::Number(Number::$fn(self as $as))
            }
        }
    };
}

to_number_impl!(i8, i64, from_i64);
to_number_impl!(u8, u64, from_u64);
to_number_impl!(i16, i64, from_i64);
to_number_impl!(u16, u64, from_u64);
to_number_impl!(i32, i64, from_i64);
to_number_impl!(u32, u64, from_u64);
to_number_impl!(i64, i64, from_i64);
to_number_impl!(u64, u64, from_u64);
to_number_impl!(f32, f64, from_f64);
to_number_impl!(f64, f64, from_f64);

// impl<'a, T: ?Sized> ToValue for &'a T
// where
//     T: ToValue,
// {
//     fn to_value(self) -> Value {
//         (*self).to_value()
//     }
// }

// impl<'a, T: ?Sized> ToValue for &'a T
// where
//     T: ToValue,
// {
//     fn to_value(self) -> Value {
//         (**self).to_value()
//     }
// }

// impl<'a, T: ?Sized> ToValue for &'a mut T
// where
//     T: ToValue,
// {
//     fn to_value(self) -> Value {
//         (*self).to_value()
//     }
// }

// impl<'a, T: ?Sized> ToValue for &'a &'a &'a &'a T
// where
//     T: ToValue,
// {
//     fn to_value(self) -> Value {
//         (****self).to_value()
//     }
// }

// impl<T> ToValue for &&T
// where
//     T: ToValue,
// {
//     fn to_value(self) -> Value {
//         self.to_value()
//     }
// }
