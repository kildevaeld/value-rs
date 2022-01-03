use std::collections::BTreeMap;

use value::{Value, ValueType};
use value_validate::TypedValidator;

// #[cfg_attr(
//     feature = "serde",
//     derive(serde_lib::Serialize, serde_lib::Deserialize)
// )]
// #[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum ParameterKind {
//     Value(ValueType),
//     Struct(BTreeMap<String, ParameterKind>),
//     Vec(Box<ParameterKind>),
// }

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug)]
pub struct Parameter {
    kind: TypedValidator,
    name: String,
    required: bool,
    default: Option<Value>,
}

pub struct Parameters {
    params: Vec<Parameter>,
}

impl Parameters {
    pub fn validate(&self, args: &Arguments) -> bool {
        true
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    args: Vec<Value>,
}
