use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use value::Value;
use value_validate::Error as ValidationError;
use value_validate::Validator;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub fn box_error<E: std::error::Error + Send + Sync + 'static>(error: E) -> BoxError {
    Box::new(error)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    kind: Validator,
    default: Option<Value>,
}

impl Parameter {
    pub fn new(kind: impl Into<Validator>) -> Parameter {
        Parameter {
            kind: kind.into(),
            default: None,
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Parameters {
    pub params: Arc<Vec<Parameter>>,
}

impl Parameters {
    pub fn add(mut self, param: Parameter) -> Self {
        let params = Arc::get_mut(&mut self.params).unwrap();
        params.push(param);
        self
    }
}

impl Parameters {
    pub fn validate(&self, _args: &Arguments) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arguments {
    args: Vec<Value>,
}

impl Arguments {
    pub fn empty() -> Arguments {
        Arguments {
            args: Vec::default(),
        }
    }
    pub fn try_get<'de, T: Deserialize<'de>>(&self, idx: usize) -> Result<T, Error> {
        if let Some(found) = self.args.get(idx) {
            Ok(found.clone().try_into::<T>().unwrap())
        } else {
            panic!("could not index: {}", idx);
        }
    }
}

impl From<Vec<Value>> for Arguments {
    fn from(v: Vec<Value>) -> Self {
        Arguments { args: v }
    }
}
