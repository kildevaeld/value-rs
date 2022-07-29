use crate::{Validation, ValidationBox, ValidationError, ValidationExt, Validator};
use core::any::Any;
use value::Value;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Item {
    validator: ValidationBox,
}

#[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
impl Validation for Item {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, _: &Value) -> Result<(), ValidationError> {
        Ok(())
    }
}

pub fn item<V: Into<Validator>>(value: V) -> Item {
    Item {
        validator: value.into().boxed(),
    }
}
