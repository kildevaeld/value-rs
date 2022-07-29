use crate::{Validation, ValidationError};
use core::any::Any;
use value::Value;
use value_types::ValueType;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct NumberSize(pub ValueType);

#[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
impl Validation for NumberSize {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, _: &Value) -> Result<(), ValidationError> {
        Ok(())
    }
}

pub fn number_kind(kind: ValueType) -> NumberSize {
    NumberSize(kind)
}
