use crate::{
    error::{self, Error},
    types::ValidationList,
    Validator,
};
use alloc::{boxed::Box, vec::Vec};
use core::any::Any;
use core::fmt::Debug;
use value::{NumberType, Value, ValueType};

pub type ValidationBox = Box<dyn Validation>;

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Validation: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn validate(&self, value: &Value) -> Result<(), Error>;
}

pub trait ValidationExt: Validation {
    fn boxed(self) -> ValidationBox
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<V> ValidationExt for V where V: Validation {}

/**
 * Required
 */

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone)]
pub struct Required;

#[cfg_attr(feature = "serde", typetag::serde(name = "required"))]
impl Validation for Required {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if value.is_none() {
            return Err(Error::Required);
        }
        Ok(())
    }
}

pub fn required() -> Required {
    Required
}

/**
 *
 * Min
 *
 */

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, Copy)]
pub struct Min(usize);

#[cfg_attr(feature = "serde", typetag::serde(name = "min"))]
impl Validation for Min {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let ret = match value {
            Value::String(str) => str.len() >= self.0,
            Value::Number(n) => (n.as_u64() as usize) >= self.0,
            Value::Bytes(bs) => bs.len() >= self.0,
            _ => false,
        };

        if !ret {
            return Err(Error::Min { min: self.0 });
        }

        Ok(())
    }
}

pub fn min(v: usize) -> Min {
    Min(v)
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, Copy)]
pub struct Max(usize);

#[cfg_attr(feature = "serde", typetag::serde(name = "max"))]
impl Validation for Max {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let ret = match value {
            Value::String(str) => str.len() <= self.0,
            Value::Number(n) => (n.as_u64() as usize) <= self.0,
            Value::Bytes(bs) => bs.len() <= self.0,
            _ => false,
        };

        if !ret {
            return Err(Error::Min { min: self.0 });
        }
        Ok(())
    }
}

pub fn max(v: usize) -> Max {
    Max(v)
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone)]
pub struct Equal(pub Value);

#[cfg_attr(feature = "serde", typetag::serde(name = "equal"))]
impl Validation for Equal {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if &self.0 != value {
            return Err(Error::Equal);
        }
        Ok(())
    }
}

pub fn equal<V: Into<Value>>(value: V) -> Equal {
    Equal(value.into())
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug)]
pub struct Tuple(pub Vec<ValidationBox>);

#[cfg_attr(feature = "serde", typetag::serde(name = "tuple"))]
impl Validation for Tuple {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let list = match value.as_list() {
            Some(list) => list,
            None => {
                return Err(Error::InvalidType {
                    expected: ValueType::List,
                    found: value.ty(),
                })
            }
        };

        if list.len() != self.0.len() {
            panic!("not equal len");
        }

        let values = self.0.iter().zip(list.iter());

        let mut errors = Vec::default();
        for (idx, (validation, value)) in values.enumerate() {
            if let Err(err) = validation.validate(value) {
                errors.push((idx, err));
            }
        }

        Ok(())
    }
}

pub fn tuple<V: ValidationList>(value: V) -> Tuple {
    Tuple(value.into_list())
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug)]
pub struct OneOf(pub Vec<ValidationBox>);

#[cfg_attr(feature = "serde", typetag::serde(name = "one_of"))]
impl Validation for OneOf {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, v: &Value) -> Result<(), Error> {
        let mut errors = Vec::default();

        for val in &self.0 {
            if let Err(err) = val.validate(v) {
                errors.push(err)
            } else {
                return Ok(());
            }
        }

        if !errors.is_empty() {
            return Err(Error::Multi(errors));
        }

        Ok(())
    }
}

pub fn one_of<V: ValidationList>(value: V) -> OneOf {
    OneOf(value.into_list())
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug)]
pub struct Item {
    validator: Validator,
}

#[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
impl Validation for Item {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, _: &Value) -> Result<(), Error> {
        Ok(())
    }
}

pub fn item<V: Into<Validator>>(value: V) -> Item {
    Item {
        validator: value.into(),
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug)]
pub struct NumberSize(pub NumberType);

#[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
impl Validation for NumberSize {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, _: &Value) -> Result<(), Error> {
        Ok(())
    }
}

pub fn number_kind(kind: NumberType) -> NumberSize {
    NumberSize(kind)
}
