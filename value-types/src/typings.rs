use std::borrow::Cow;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<S: AsRef<str>> TypeDef<S> {
    pub fn is_like<O: AsRef<str>>(&self, other: &TypeDef<O>) -> bool {
        match (self, other) {
            (TypeDef::Value(v), TypeDef::Value(o)) => v == o,
            (TypeDef::Struct(v), TypeDef::Struct(o)) => v.is_like(o),
            (TypeDef::Tuple(v), TypeDef::Tuple(o)) => v.is_like(o),
            (TypeDef::Union(v), TypeDef::Union(o)) => v.is_like(o),
            (TypeDef::Enum(v), TypeDef::Enum(o)) => v.is_like(o),
            (TypeDef::Optional(v), TypeDef::Optional(o)) => v.is_like(o),
            (TypeDef::Value(ValueType::Map), TypeDef::Struct(_)) => true,
            (TypeDef::Value(ValueType::List), TypeDef::Tuple(_)) => true,
            (TypeDef::Union(v), o) => {
                for field in &v.types {
                    if field.is_like(o) {
                        return true;
                    }
                }
                false
            }
            (TypeDef::Optional(_), TypeDef::Value(ValueType::None)) => true,
            _ => false,
        }
    }
}

impl<S> TypeDef<S> {
    pub fn is_optional(&self) -> bool {
        match self {
            TypeDef::Optional(_) => true,
            TypeDef::Value(ValueType::None) => true,
            _ => false,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructDef<S> {
    name: Option<S>,
    fields: Vec<(S, TypeDef<S>)>,
}

impl<S> StructDef<S> {
    pub fn new(name: impl Into<Option<S>>) -> StructDef<S> {
        StructDef {
            name: name.into(),
            fields: Vec::default(),
        }
    }
    pub fn with_field(mut self, name: impl Into<S>, field: TypeDef<S>) -> Self {
        self.fields.push((name.into(), field));
        self
    }

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

impl<S: AsRef<str>> StructDef<S> {
    pub fn is_like<O: AsRef<str>>(&self, other: &StructDef<O>) -> bool {
        for field in &self.fields {
            let found = other
                .fields()
                .iter()
                .find(|n| n.0.as_ref() == field.0.as_ref());

            if let Some(found) = found {
                if !field.1.is_like(&found.1) {
                    return false;
                }
            } else if field.1.is_optional() {
                continue;
            } else {
                return false;
            }
        }

        true
    }
}

impl<S> From<StructDef<S>> for TypeDef<S> {
    fn from(s: StructDef<S>) -> Self {
        TypeDef::Struct(s)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TupleDef<S> {
    name: Option<S>,
    members: Vec<TypeDef<S>>,
}

impl<S: Into<Cow<'static, str>>> TupleDef<S> {
    pub fn to_owned(self) -> TupleDef<Cow<'static, str>> {
        TupleDef {
            name: self.name.map(|m| m.into()),
            members: self.members.into_iter().map(|m| m.to_owned()).collect(),
        }
    }
}

impl<S: AsRef<str>> TupleDef<S> {
    pub fn is_like<O: AsRef<str>>(&self, other: &TupleDef<O>) -> bool {
        for (idx, field) in self.members.iter().enumerate() {
            let member = match other.members.get(idx) {
                Some(m) => m,
                None => {
                    if field.is_optional() {
                        continue;
                    }
                    return false;
                }
            };

            if !field.is_like(member) {
                return false;
            }
        }

        true
    }
}

impl<S> From<TupleDef<S>> for TypeDef<S> {
    fn from(s: TupleDef<S>) -> Self {
        TypeDef::Tuple(s)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<S: AsRef<str>> UnionDef<S> {
    pub fn is_like<O: AsRef<str>>(&self, other: &UnionDef<O>) -> bool {
        for (idx, next) in other.types.iter().enumerate() {
            let ty = match self.types.get(idx) {
                Some(ty) => ty,
                None => return false,
            };

            if !next.is_like(ty) {
                return false;
            }
        }

        true
    }
}

impl<S> From<UnionDef<S>> for TypeDef<S> {
    fn from(s: UnionDef<S>) -> Self {
        TypeDef::Union(s)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<S: AsRef<str>> EnumDef<S> {
    pub fn is_like<O: AsRef<str>>(&self, other: &EnumDef<O>) -> bool {
        for (idx, next) in other.types.iter().enumerate() {
            let ty = match self.types.get(idx) {
                Some(ty) => ty,
                None => return false,
            };

            if next.0.as_ref() != ty.0.as_ref() {
                return false;
            }

            if !next.1.is_like(&ty.1) {
                return false;
            }
        }

        true
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

#[cfg(test)]
mod test {

    use value::Map;

    use crate::{HasType, HasTypeDef};

    use super::*;

    #[test]
    fn test() {
        let map_ty = Map::typed();
        let mut m = Map::default();

        m.insert("name", "Rasmus");
        m.insert("age", 38i32);

        assert!(map_ty.is_like(&m.type_def()))
    }

    #[test]
    fn test2() {
        let struct_ty = StructDef::<&'static str>::new(None)
            .with_field("name", ValueType::String.into())
            .with_field("age", ValueType::I32.into());

        let mut m = Map::default();

        m.insert("name", "Rasmus");
        m.insert("age", 38i32);

        assert!(TypeDef::Struct(struct_ty).is_like(&m.type_def()))
    }
}
