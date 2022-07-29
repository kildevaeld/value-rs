use crate::{
    any_of, item, ListValidator, NumberValidator, ObjectValidator, StringValidator, Validatable,
    Validator, ValidatorBuilderExt,
};
use alloc::{collections::BTreeMap, vec, vec::Vec};
use value_types::ValueType;

macro_rules! validatable_number {
    ($($ty: ident => $kind: ident),*) => {
        $(
            impl Validatable for $ty {
                fn validator() -> Validator {
                    NumberValidator::default().kind(ValueType::$kind).into()
                }
            }

        )*
    };
}

validatable_number!(
    u8 => U8,
    i8 => I8,
    u16 => U16,
    i16 => I16,
    u32 => U32,
    i32 => I32,
    i64 => I64,
    u64 => U64,
    f32 => F32,
    f64 => F64,
    usize => U64,
    isize => I64
);

impl<'a> Validatable for &'a str {
    fn validator() -> Validator {
        StringValidator::default().into()
    }
}

impl Validatable for alloc::string::String {
    fn validator() -> Validator {
        StringValidator::default().into()
    }
}

impl<K, V: Validatable> Validatable for BTreeMap<K, V> {
    fn validator() -> Validator {
        ObjectValidator::default().and(item(V::validator())).into()
    }
}

impl<V: Validatable> Validatable for Vec<V> {
    fn validator() -> Validator {
        ListValidator::default().and(item(V::validator())).into()
    }
}

impl<V> Validatable for Option<V>
where
    V: Validatable,
{
    fn validator() -> Validator {
        any_of(vec![V::validator()]).into()
    }
}
