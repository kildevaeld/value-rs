use crate::{validations::Validations, Validator};
use value::ValueType;

pub struct StringValidator {
    v: Validations<String>,
}

impl StringValidator {}

impl Validator for StringValidator {
    type Type = String;
    fn validations_mut(&mut self) -> &mut Validations<Self::Type> {
        &mut self.v
    }
    fn validations(&self) -> &Validations<Self::Type> {
        &self.v
    }
}
