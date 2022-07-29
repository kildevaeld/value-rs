use crate::{Validation, ValidationError};
use core::any::Any;
use value::Value;

/**
 * Required
 */

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Required;

#[cfg_attr(feature = "serde", typetag::serde(name = "required"))]
impl Validation for Required {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        if value.is_none() {
            return Err(ValidationError::Required);
        }
        Ok(())
    }
}

pub fn required() -> Required {
    Required
}
