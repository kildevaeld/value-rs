use core::fmt;

use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};

use value::{Value, ValueType};

use crate::error::Error;

#[cfg_attr(feature = "serde", typetag::serde(tag = "$type"))]
pub trait Validation: Send + Sync + fmt::Debug {
    fn validate(&self, value: &Value) -> Result<(), Error>;
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]
pub struct Alt(Vec<TypedValidator>);

impl Alt {
    pub fn or(mut self, v: TypedValidator) -> Self {
        self.0.push(v);
        self
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Validation for Alt {
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let mut errors = Vec::default();
        for validator in &self.0 {
            if let Err(err) = validator.validate(value) {
                errors.push(err)
            } else {
                return Ok(());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

// Implem

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug)]
pub struct TypedValidator {
    #[cfg_attr(feature = "serde", serde(rename = "$type"))]
    ty: ValueType,
    validators: Vec<Box<dyn Validation>>,
}

impl TypedValidator {
    pub fn new(ty: ValueType) -> TypedValidator {
        TypedValidator {
            ty,
            validators: Vec::default(),
        }
    }
    pub fn ty(&self) -> ValueType {
        self.ty
    }

    pub fn and<V: Validation + 'static>(mut self, val: V) -> Self {
        self.validators.push(Box::new(val));
        self
    }

    pub fn push<V: Validation + 'static>(&mut self, val: V) -> &mut Self {
        self.validators.push(Box::new(val));
        self
    }
}

#[cfg_attr(feature = "serde", typetag::serde(name = "type"))]
impl Validation for TypedValidator {
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let mut errors = Vec::default();
        if value.ty() != self.ty
            && value.ty() != ValueType::None
            && !(value.is_number() && self.ty.is_number())
        {
            errors.push(Error::InvalidType {
                expected: self.ty,
                found: value.ty(),
            });
        }

        for validator in &self.validators {
            validator.validate(value)?;
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Debug, Clone, Copy)]
pub struct Min(usize);

#[cfg_attr(feature = "serde", typetag::serde)]
impl Validation for Min {
    fn validate(&self, value: &Value) -> Result<(), Error> {
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

#[cfg_attr(feature = "serde", typetag::serde)]
impl Validation for Max {
    fn validate(&self, value: &Value) -> Result<(), Error> {
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
pub struct OneOf(Vec<Value>);

#[cfg_attr(feature = "serde", typetag::serde)]
impl Validation for OneOf {
    fn validate(&self, value: &Value) -> Result<(), Error> {
        for one in &self.0 {
            if value == one {
                return Ok(());
            }
        }
        Ok(())
    }
}

pub fn oneof(v: Vec<Value>) -> OneOf {
    OneOf(v)
}
