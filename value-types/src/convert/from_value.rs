use std::{
    collections::{BTreeMap, HashMap},
    convert::Infallible,
};

use value::{FromValueErr, Value};

pub trait FromValue: Sized {
    type Error;
    fn from_value(value: Value) -> Result<Self, Self::Error>;
}

pub trait TryValueInto<V>: Sized {
    type Error;
    fn try_into(self) -> Result<V, Self::Error>;
}

impl<V> TryValueInto<V> for Value
where
    V: FromValue,
{
    type Error = V::Error;

    fn try_into(self) -> Result<V, Self::Error> {
        V::from_value(self)
    }
}

macro_rules! from_value {
    ($ty: ty, $method: ident) => {
        impl FromValue for $ty {
            type Error = FromValueErr<'static>;
            fn from_value(value: Value) -> Result<Self, Self::Error> {
                match value.$method() {
                    Ok(ret) => Ok(ret),
                    Err(err) => Err(FromValueErr::Value(err)),
                }
            }
        }
    };
}

from_value!(String, into_string);
from_value!(Vec<u8>, into_bytes);
from_value!(bool, into_bool);
from_value!(char, into_char);

impl<'a, T> FromValue for Vec<T>
where
    T: FromValue<Error = FromValueErr<'a>>,
{
    type Error = T::Error;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        let list = value.into_list().map_err(FromValueErr::Value)?;

        list.into_iter()
            .map(T::from_value)
            .collect::<Result<_, _>>()
    }
}

impl<'a, T> FromValue for BTreeMap<String, T>
where
    T: FromValue<Error = FromValueErr<'a>>,
{
    type Error = T::Error;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        let list = value.into_map().map_err(FromValueErr::Value)?;

        list.into_iter()
            .map(|(name, item)| match T::from_value(item) {
                Ok(ret) => Ok((name, ret)),
                Err(err) => Err(err),
            })
            .collect::<Result<_, _>>()
    }
}

impl<'a, T> FromValue for HashMap<String, T>
where
    T: FromValue<Error = FromValueErr<'a>>,
{
    type Error = T::Error;

    fn from_value(value: Value) -> Result<Self, Self::Error> {
        let list = value.into_map().map_err(FromValueErr::Value)?;

        list.into_iter()
            .map(|(name, item)| match T::from_value(item) {
                Ok(ret) => Ok((name, ret)),
                Err(err) => Err(err),
            })
            .collect::<Result<_, _>>()
    }
}

impl FromValue for Value {
    type Error = Infallible;
    fn from_value(value: Value) -> Result<Self, Self::Error> {
        Ok(value)
    }
}

// //

pub trait FromValueRef<'a>: Sized {
    type Error;
    fn from_value(value: &'a Value) -> Result<Self, Self::Error>;
}

pub trait TryIntoRef<'a, V>: Sized {
    type Error;
    fn try_into_ref(&'a self) -> Result<V, Self::Error>;
}

impl<'a, V> TryIntoRef<'a, V> for Value
where
    V: FromValueRef<'a>,
{
    type Error = V::Error;

    fn try_into_ref(&'a self) -> Result<V, Self::Error> {
        V::from_value(self)
    }
}

impl<'a, T> FromValueRef<'a> for &'a T
where
    &'a T: TryFrom<&'a Value>,
{
    type Error = <&'a T as TryFrom<&'a value::Value>>::Error;

    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        value.try_into()
    }
}

impl<'a> FromValueRef<'a> for &'a str {
    type Error = FromValueErr<'a>;
    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        value.try_into()
    }
}

// impl<'a> FromValueRef<'a> for &'a [u8] {
//     type Error = FromValueErr<'a>;
//     fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
//         value.try_into()
//     }
// }

impl<'a> FromValueRef<'a> for String {
    type Error = FromValueErr<'a>;
    fn from_value(value: &'a Value) -> Result<Self, Self::Error> {
        <&'a String as FromValueRef<'a>>::from_value(value).map(|m| m.to_string())
    }
}
