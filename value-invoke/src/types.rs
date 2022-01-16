use std::any::TypeId;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::error::Error;
use value::{Value, ValueType};
use value_validate::Error as ValidationError;
use value_validate::TypedValidator;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
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
    default: Option<Value>,
}

impl Parameter {
    pub fn new(name: impl ToString, kind: TypedValidator) -> Parameter {
        Parameter {
            kind: kind,
            name: name.to_string(),
            default: None,
        }
    }
}

#[derive(Default)]
pub struct Parameters {
    params: Vec<Parameter>,
}

impl Parameters {
    pub fn add(mut self, param: Parameter) -> Self {
        self.params.push(param);
        self
    }
}

impl Parameters {
    pub fn validate(&self, args: &Arguments) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    args: HashMap<String, Value>,
}

impl Arguments {
    #[cfg(feature = "serde")]
    pub fn try_get<'de, T: serde_lib::de::Deserialize<'de>>(self, name: &str) -> Result<T, Error> {
        if let Some(found) = self.args.get(name) {
            found.clone().try_into()?
        } else {
            panic!("")
        }
    }
}

impl From<Vec<Value>> for Arguments {
    fn from(v: Vec<Value>) -> Self {
        Arguments { args: v }
    }
}
