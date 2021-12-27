use std::collections::BTreeMap;

use value::{Value, ValueType};

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterKind {
    Value(ValueType),
    Struct(BTreeMap<String, ParameterKind>),
    Vec(Box<ParameterKind>),
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Parameter {
    kind: ParameterKind,
    name: String,
    required: bool,
    default: Option<Value>,
}

pub struct Parameters {
    params: Vec<Parameter>,
}

impl Parameters {}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    args: Vec<Value>,
}
