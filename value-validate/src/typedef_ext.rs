use value::Value;
use value_types::{EnumDef, HasType, StructDef, TypeDef, UnionDef, ValueType};

use crate::{
    any_of, item, tuple, BoolValidator, ListValidator, ObjectValidator, StringValidator, Validator,
    ValidatorBuilder, ValidatorBuilderCommon, ValidatorBuilderExt,
};

mod sealed {
    use value_types::{EnumDef, StructDef, TypeDef, UnionDef, ValueType};

    pub trait Sealed {}

    impl<S> Sealed for TypeDef<S> {}

    impl<S> Sealed for StructDef<S> {}

    impl<S> Sealed for UnionDef<S> {}

    impl<S> Sealed for EnumDef<S> {}

    impl Sealed for ValueType {}
}

pub trait TypeDefExt: sealed::Sealed {
    fn create_validator(&self) -> Validator;
}

impl<S> TypeDefExt for TypeDef<S>
where
    S: AsRef<str>,
{
    fn create_validator(&self) -> Validator {
        match self {
            TypeDef::Struct(s) => s.create_validator(),
            TypeDef::Value(v) => v.create_validator(),
            TypeDef::Union(v) => v.create_validator(),
            _ => panic!(),
        }
    }
}

impl<S> TypeDefExt for StructDef<S>
where
    S: AsRef<str>,
{
    fn create_validator(&self) -> Validator {
        let mut o = ObjectValidator::default();

        for (name, field) in self.fields() {
            o = o.field(name.as_ref(), field.create_validator());
        }

        o.into()
    }
}

impl<S> TypeDefExt for UnionDef<S>
where
    S: AsRef<str>,
{
    fn create_validator(&self) -> Validator {
        let vals = self.types().iter().map(|m| m.create_validator()).collect();
        any_of(vals).into()
    }
}

impl<S> TypeDefExt for EnumDef<S>
where
    S: AsRef<str>,
{
    fn create_validator(&self) -> Validator {
        let vals = self
            .variants()
            .iter()
            .map(|(s, m)| {
                let validator: Validator = ListValidator::default()
                    .and(item(any_of(vec![
                        StringValidator::default()
                            .equal(s.as_ref().to_string())
                            .into(),
                        m.create_validator(),
                    ])))
                    .into();

                validator
            })
            .collect();
        any_of(vals).into()
    }
}

impl TypeDefExt for ValueType {
    fn create_validator(&self) -> Validator {
        match self {
            ValueType::Bool => BoolValidator::default().into(),
            ValueType::String => StringValidator::default().into(),
            _ => panic!(),
        }
    }
}

pub trait HasTypeExt: HasType {
    fn validator() -> Validator {
        Self::typed().create_validator()
    }
}

impl<T> HasTypeExt for T where T: HasType {}
