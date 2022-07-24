use core::convert::Infallible;
use value::{Number, Value};

pub trait IntoValue {
    type Error;
    fn into_value(self) -> Result<Value, Self::Error>;
}

macro_rules! to_value {
    ($ty: ty, $variant: ident) => {
        impl IntoValue for $ty {
            type Error = Infallible;
            fn into_value(self) -> Result<Value, Self::Error> {
                Ok(Value::$variant(self.into()))
            }
        }
    };
    ($variant: ident, $($ty: ty)*) => {
        $(
            impl IntoValue for $ty {
                type Error = Infallible;
                fn into_value(self) -> Result<Value, Self::Error> {
                    Ok(Value::$variant(self.into()))
                }
            }
        )*
    };
}

impl<'a, S: Clone + IntoValue> IntoValue for &'a S {
    type Error = S::Error;

    fn into_value(self) -> Result<Value, Self::Error> {
        self.clone().into_value()
    }
}

impl<'a> IntoValue for &'a str {
    type Error = Infallible;
    fn into_value(self) -> Result<Value, Self::Error> {
        self.to_string().into_value()
    }
}

to_value!(String, String);
to_value!(bool, Bool);
to_value!(Vec<u8>, Bytes);
to_value!(Number, u8 i8 u16 i16 u32 i32 u64 i64);

#[cfg(feature = "serde")]
impl<T> IntoValue for super::Serde<T>
where
    T: serde::Serialize,
{
    type Error = crate::ser::SerializerError;
    fn into_value(self) -> Result<Value, Self::Error> {
        crate::to_value(self.0)
    }
}

#[cfg(feature = "serde")]
impl<T> From<T> for super::Serde<T>
where
    T: serde::Serialize,
{
    fn from(v: T) -> super::Serde<T> {
        super::Serde(v)
    }
}

impl IntoValue for Value {
    type Error = Infallible;

    fn into_value(self) -> Result<Value, Self::Error> {
        Ok(self)
    }
}

impl IntoValue for Number {
    type Error = Infallible;

    fn into_value(self) -> Result<Value, Self::Error> {
        Ok(self.into())
    }
}
