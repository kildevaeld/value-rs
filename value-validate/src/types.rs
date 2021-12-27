use value::{NumberType, ValueType};

use crate::Error;

pub trait Validation<I: ?Sized> {
    fn validate(&self, input: &I) -> Result<(), Error>;
}

pub trait Typed {
    fn typed() -> ValueType;
}

macro_rules! ty_impl {
    ($name: ident, $ty: ident) => {
        impl Typed for $name {
            fn typed() -> ValueType {
                ValueType::$ty
            }
        }
    };
    ($name: ident, number $ty: ident) => {
        impl Typed for $name {
            fn typed() -> ValueType {
                ValueType::Number(NumberType::$ty)
            }
        }
    };
}

ty_impl!(i8, number I8);
ty_impl!(u8, number U8);
ty_impl!(i16, number I16);
ty_impl!(u16, number U16);
ty_impl!(i32, number I32);
ty_impl!(u32, number U32);
ty_impl!(i64, number I64);
ty_impl!(u64, number U64);
ty_impl!(f32, number F32);
ty_impl!(f64, number F64);
ty_impl!(String, String);
ty_impl!(bool, Bool);
