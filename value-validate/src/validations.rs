use std::borrow::Borrow;

use crate::Error;

use valid::Validation;

#[derive(Default)]
pub struct Validations<S>(Vec<Box<dyn Validation<S> + Send + Sync>>);

impl<S> Validations<S> {
    pub fn new() -> Validations<S> {
        Validations(Vec::default())
    }

    pub fn push<V: Validation<S> + Send + Sync + 'static>(&mut self, validation: V) -> &mut Self {
        self.0.push(Box::new(validation));
        self
    }

    pub fn validate(&self, value: &S) -> Result<(), Error> {
        for v in self.0.iter() {
            v.validate(value)?;
        }

        Ok(())
    }
}
