use value::{Number, Value};

use crate::ValueType;

mod sealed {
    pub trait Sealed {}

    impl Sealed for value::Value {}

    impl Sealed for value::Number {}
}

pub trait ValueExt: sealed::Sealed {
    fn ty(&self) -> ValueType;
    fn is(&self, ty: ValueType) -> bool {
        self.ty() == ty
    }
}

impl ValueExt for Value {
    fn ty(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Number(n) => n.ty(),
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
}

impl ValueExt for Number {
    fn ty(&self) -> ValueType {
        match *self {
            Number::U8(_) => ValueType::U8,
            Number::I8(_) => ValueType::I8,
            Number::U16(_) => ValueType::U16,
            Number::I16(_) => ValueType::I16,
            Number::I32(_) => ValueType::I32,
            Number::U32(_) => ValueType::U32,
            Number::I64(_) => ValueType::I64,
            Number::U64(_) => ValueType::U64,
            #[cfg(feature = "ordered_float")]
            Number::F32(_) => ValueType::F32,
            #[cfg(feature = "ordered_float")]
            Number::F64(_) => ValueType::F64,
            #[cfg(not(feature = "ordered_float"))]
            Number::F32(_) => ValueType::F32,
            #[cfg(not(feature = "ordered_float"))]
            Number::F64(_) => ValueType::F64,
        }
    }
}
