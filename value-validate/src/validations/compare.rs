use crate::{Validation, ValidationError};
use core::any::Any;
use value::Value;

/**
 *
 * Min
 *
 */

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Min(usize);

#[cfg_attr(feature = "serde", typetag::serde(name = "min"))]
impl Validation for Min {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let ret = match value {
            Value::String(str) => str.len() >= self.0,
            Value::Number(n) => (n.as_u64() as usize) >= self.0,
            Value::Bytes(bs) => bs.len() >= self.0,
            _ => false,
        };

        if !ret {
            return Err(ValidationError::Compare {
                expected: (self.0 as u64).into(),
                found: value.clone(),
                operator: crate::Operator::Min,
            });
        }

        Ok(())
    }
}

pub fn min(v: usize) -> Min {
    Min(v)
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Max(usize);

#[cfg_attr(feature = "serde", typetag::serde(name = "max"))]
impl Validation for Max {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let ret = match value {
            Value::String(str) => str.len() <= self.0,
            Value::Number(n) => (n.as_u64() as usize) <= self.0,
            Value::Bytes(bs) => bs.len() <= self.0,
            _ => false,
        };

        if !ret {
            return Err(ValidationError::Compare {
                expected: (self.0 as u64).into(),
                found: value.clone(),
                operator: crate::Operator::Min,
            });
        }
        Ok(())
    }
}

pub fn max(v: usize) -> Max {
    Max(v)
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Equal(pub Value);

#[cfg_attr(feature = "serde", typetag::serde(name = "equal"))]
impl Validation for Equal {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        if &self.0 != value {
            return Err(ValidationError::Compare {
                expected: self.0.clone(),
                found: value.clone(),
                operator: crate::Operator::Equal,
            });
        }
        Ok(())
    }
}

pub fn equal<V: Into<Value>>(value: V) -> Equal {
    Equal(value.into())
}
