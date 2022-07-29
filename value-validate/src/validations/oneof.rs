use crate::{types::ValidationList, Validation, ValidationBox, ValidationError};
use core::any::Any;
use value::Value;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct OneOf(pub Vec<ValidationBox>);

#[cfg_attr(feature = "serde", typetag::serde(name = "one_of"))]
impl Validation for OneOf {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, v: &Value) -> Result<(), ValidationError> {
        let mut errors = Vec::default();

        for val in &self.0 {
            if let Err(err) = val.validate(v) {
                errors.push(err)
            } else {
                return Ok(());
            }
        }

        if !errors.is_empty() {
            return Err(ValidationError::OneOf(errors));
        }

        Ok(())
    }
}

pub fn one_of<V: ValidationList>(value: V) -> OneOf {
    OneOf(value.into_list())
}
