use std::borrow::Cow;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeDef<S> {
    Struct(StructDef<S>),
    Tuple(TupleDef<S>),
    Union(UnionDef<S>),
    Enum(EnumDef<S>),
    Value(ValueType),
    Optional(Box<TypeDef<S>>),
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
            TypeDef::Union(u) => TypeDef::Union(u.to_owned()),
            TypeDef::Enum(e) => TypeDef::Enum(e.to_owned()),
            TypeDef::Optional(o) => TypeDef::Optional(Box::new(o.to_owned())),
            TypeDef::Value(v) => TypeDef::Value(v),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructDef<S> {
    name: Option<S>,
    fields: Vec<(S, TypeDef<S>)>,
}

impl<S> StructDef<S> {
    pub fn fields(&self) -> &Vec<(S, TypeDef<S>)> {
        &self.fields
    }
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnionDef<S> {
    types: Vec<TypeDef<S>>,
}

impl<S> UnionDef<S> {
    pub fn types(&self) -> &Vec<TypeDef<S>> {
        &self.types
    }
}

impl<S: Into<Cow<'static, str>>> UnionDef<S> {
    pub fn to_owned(self) -> UnionDef<Cow<'static, str>> {
        UnionDef {
            types: self.types.into_iter().map(|m| m.to_owned()).collect(),
        }
    }
}

impl<S> From<UnionDef<S>> for TypeDef<S> {
    fn from(s: UnionDef<S>) -> Self {
        TypeDef::Union(s)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumDef<S> {
    types: Vec<(S, TypeDef<S>)>,
}

impl<S> EnumDef<S> {
    pub fn variants(&self) -> &Vec<(S, TypeDef<S>)> {
        &self.types
    }
}

impl<S: Into<Cow<'static, str>>> EnumDef<S> {
    pub fn to_owned(self) -> EnumDef<Cow<'static, str>> {
        EnumDef {
            types: self
                .types
                .into_iter()
                .map(|(n, m)| (n.into(), m.to_owned()))
                .collect(),
        }
    }
}

impl<S> From<EnumDef<S>> for TypeDef<S> {
    fn from(s: EnumDef<S>) -> Self {
        TypeDef::Enum(s)
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
    pub fn is_number(&self) -> bool {
        match self {
            ValueType::U8 | ValueType::I8 => true,
            _ => false,
        }
    }
}
