use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeDef<S> {
    Struct(StructDef<S>),
    Tuple(TupleDef<S>),
    Value(ValueType),
}

impl<S> From<ValueType> for TypeDef<S> {
    fn from(v: ValueType) -> Self {
        TypeDef::Value(v)
    }
}

impl<S: Into<Cow<'static, str>>> TypeDef<S> {
    pub fn to_owned(self) -> TypeDef<Cow<'static, str>> {
        match self {
            TypeDef::Struct(s) => TypeDef::Struct(s.to_owned()),
            TypeDef::Tuple(t) => TypeDef::Tuple(t.to_owned()),
            TypeDef::Value(v) => TypeDef::Value(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructDef<S> {
    name: Option<S>,
    fields: Vec<(S, TypeDef<S>)>,
}

impl<S: Into<Cow<'static, str>>> StructDef<S> {
    pub fn to_owned(self) -> StructDef<Cow<'static, str>> {
        StructDef {
            name: self.name.map(|m| m.into()),
            fields: self
                .fields
                .into_iter()
                .map(|(name, field)| (name.into(), field.to_owned()))
                .collect(),
        }
    }
}

impl<S> From<StructDef<S>> for TypeDef<S> {
    fn from(s: StructDef<S>) -> Self {
        TypeDef::Struct(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TupleDef<S> {
    name: Option<S>,
    fields: Vec<TypeDef<S>>,
}

impl<S: Into<Cow<'static, str>>> TupleDef<S> {
    pub fn to_owned(self) -> TupleDef<Cow<'static, str>> {
        TupleDef {
            name: self.name.map(|m| m.into()),
            fields: self.fields.into_iter().map(|m| m.to_owned()).collect(),
        }
    }
}

impl<S> From<TupleDef<S>> for TypeDef<S> {
    fn from(s: TupleDef<S>) -> Self {
        TypeDef::Tuple(s)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ValueType {
    // Numbers
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,

    Bool,
    Char,
    String,
    List,
    Map,
    Bytes,
    None,
    #[cfg(feature = "datetime")]
    Date,
    #[cfg(feature = "datetime")]
    DateTime,
}

impl ValueType {
    // pub fn can_cast(&self, ty: &ValueType) -> bool {
    //     use ValueType::*;
    //     if self == ty {
    //         return true;
    //     }

    //     match (*self, *ty) {
    //         (Number, Number | String | Bool | Char) => true,
    //         #[cfg(feature = "datetime")]
    //         (DateTime | Date, Date | DateTime | String) => true,
    //         (Bool, Number | String | Char) => true,
    //         (Char, Bool | Number | String) => true,
    //         (Map | List | String, Bool) => true,
    //         (String, Bytes) => true,
    //         (Map, List) => true,
    //         (_, List) => true,
    //         _ => false,
    //     }
    // }

    pub fn is_number(&self) -> bool {
        match self {
            ValueType::U8 | ValueType::I8 => true,
            _ => false,
        }
    }
}
