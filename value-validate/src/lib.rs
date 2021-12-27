mod error;
mod string;
// mod types;
// mod validation_impl;
mod validations;
use valid::Validation;
use validations::Validations;
use value::Value;

pub use self::error::Error;

pub trait Validator {
    type Type;
    fn validations_mut(&mut self) -> &mut Validations<Self::Type>;
    fn validations(&self) -> &Validations<Self::Type>;
    fn validate(&self, value: &Value) -> Result<(), Error>;
}

pub trait ValidatorExt: Validator + Sized {
    fn with<S: Validation<Self::Type> + Send + Sync + 'static>(mut self, validation: S) -> Self {
        self.validations_mut().push(validation);
        self
    }
}

pub struct StringValidator(Validations<String>);

impl Validator for StringValidator {
    type Type = String;
    fn validations_mut(&mut self) -> &mut Validations<Self::Type> {
        &mut self.0
    }
    fn validations(&self) -> &Validations<Self::Type> {
        &self.0
    }

    fn validate(&self, value: &Value) -> Result<(), Error> {
        let value = match value.as_string() {
            Some(s) => s,
            None => panic!("invalid type"),
        };
        self.validations().validate(value)
    }
}
