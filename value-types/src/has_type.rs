use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    typings::{TypeDef, ValueType},
    StructDef, ValueExt,
};
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

pub trait HasTypeDef<'a> {
    type String: Into<Cow<'a, str>>;
    fn type_def(&'a self) -> TypeDef<Self::String>;
}

impl<'a> HasTypeDef<'a> for Value {
    type String = &'a String;
    fn type_def(&'a self) -> TypeDef<&'a String> {
        match self {
            Value::Map(map) => map.type_def(),
            _ => self.ty().into(),
        }
    }
}

impl<'a> HasTypeDef<'a> for Map {
    type String = &'a String;
    fn type_def(&'a self) -> TypeDef<&'a String> {
        let mut st = StructDef::new(None);

        for (k, v) in self.iter() {
            let field = v.type_def();
            st = st.with_field(k, field);
        }

        st.into()
    }
}

macro_rules! has_type_def {
    ($($name: ty)*) => {
        $(
            impl<'a> HasTypeDef<'a> for $name {
                type String = &'a str;
                fn type_def(&'a self) -> TypeDef<&'a str> {
                    <$name as HasType>::typed().into()
                }
            }
        )*
    };
}

has_type_def!(i8 u8 i16 u16 i32 u32 i64 u64 String bool BTreeMap<String, Value>);
