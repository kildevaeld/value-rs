use crate::{Validation, ValidationError};
use core::any::Any;
use value::Value;
use value_types::{ValueExt, ValueType};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct TypeValidation(ValueType);

#[cfg_attr(feature = "serde", typetag::serde(name = "type"))]
impl Validation for TypeValidation {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        if value.ty() != self.0 {
            Err(ValidationError::InvalidType {
                expected: self.0.into(),
                found: value.ty().into(),
            })
        } else {
            Ok(())
        }
    }
}

pub fn typed(ty: ValueType) -> TypeValidation {
    TypeValidation(ty)
}
