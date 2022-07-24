use std::collections::BTreeMap;

use crate::typings::{TypeDef, ValueType};
use value::{Map, Value};

pub trait HasType {
    fn typed() -> TypeDef<&'static str>;
}

impl<'a, T> HasType for &'a T
where
    T: HasType,
{
    fn typed() -> TypeDef<&'static str> {
        T::typed()
    }
}

impl<'a> HasType for &'a str {
    fn typed() -> TypeDef<&'static str> {
        ValueType::String.into()
    }
}

macro_rules! ty_impl {
    ($name: ty, $ty: ident) => {
        impl HasType for $name {
            fn typed() -> TypeDef<&'static str> {
                ValueType::$ty.into()
            }
        }
    };
}

ty_impl!(i8, I8);
ty_impl!(u8, U8);
ty_impl!(i16, I16);
ty_impl!(u16, U16);
ty_impl!(i32, I32);
ty_impl!(u32, U32);
ty_impl!(i64, I64);
ty_impl!(u64, U64);
ty_impl!(f32, F32);
ty_impl!(f64, F64);
ty_impl!(String, String);
ty_impl!(bool, Bool);
ty_impl!(BTreeMap<String, Value>, Map);
ty_impl!(Map, Map);
ty_impl!(Vec<Value>, List);
ty_impl!(Vec<u8>, Bytes);
ty_impl!((), None);
