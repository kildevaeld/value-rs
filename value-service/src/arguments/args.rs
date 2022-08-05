use crate::errors::ArgumentError;
use value::Value;
use value_types::{FromValue, FromValueRef, HasTypeDef, IntoValue, TypeDef};

#[derive(Default)]
pub struct Arguments {
    args: Vec<Value>,
}

impl Arguments {
    pub fn build() -> ArgumentsBuilder {
        ArgumentsBuilder {
            args: Vec::default(),
        }
    }
}

impl Arguments {
    pub fn try_get_ref<'a, V: FromValueRef<'a>>(&'a self, idx: usize) -> Result<V, ArgumentError>
    where
        V::Error: Into<ArgumentError>,
    {
        let val = match self.args.get(idx) {
            Some(ret) => ret,
            None => {
                return Err(ArgumentError::Missing {
                    index: idx,
                    arity: self.args.len(),
                })
            }
        };

        V::from_value(val).map_err(|err| err.into())
    }

    pub fn try_get<V: FromValue>(&self, idx: usize) -> Result<V, ArgumentError>
    where
        V::Error: Into<ArgumentError>,
    {
        let val = match self.args.get(idx) {
            Some(ret) => ret,
            None => {
                return Err(ArgumentError::Missing {
                    index: idx,
                    arity: self.args.len(),
                })
            }
        };

        V::from_value(val.clone()).map_err(|err| err.into())
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn types<'a>(&'a self) -> Vec<TypeDef<&'a String>> {
        self.args.iter().map(|m| m.type_def()).collect()
    }
}

pub struct ArgumentsBuilder {
    args: Vec<Value>,
}

impl ArgumentsBuilder {
    pub fn with<V: IntoValue>(mut self, value: V) -> Result<Self, V::Error> {
        self.args.push(value.into_value()?);
        Ok(self)
    }

    pub fn add<V: IntoValue>(&mut self, value: V) -> Result<&mut Self, V::Error> {
        self.args.push(value.into_value()?);
        Ok(self)
    }

    pub fn build(self) -> Arguments {
        Arguments { args: self.args }
    }
}
