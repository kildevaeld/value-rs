use std::{borrow::Cow, sync::Arc};

use value_types::TypeDef;

use crate::{arguments::Arguments, errors::ValidationError};

#[derive(Debug, Default, Clone)]
pub struct Parameters {
    vec: Arc<Vec<TypeDef<Cow<'static, str>>>>,
}

impl Parameters {
    pub fn build() -> ParametersBuilder {
        ParametersBuilder {
            params: Vec::default(),
        }
    }

    pub fn get(&self, idx: usize) -> Option<&TypeDef<Cow<'static, str>>> {
        self.vec.get(idx)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a TypeDef<Cow<'static, str>>> {
        self.vec.iter()
    }

    pub fn validate(&self, args: &Arguments) -> Result<(), ValidationError> {
        Ok(())
    }
}

pub struct ParametersBuilder {
    params: Vec<TypeDef<Cow<'static, str>>>,
}

impl ParametersBuilder {
    pub fn with<S: Into<Cow<'static, str>>>(mut self, param: TypeDef<S>) -> Self {
        self.add(param);
        self
    }

    pub fn add<S: Into<Cow<'static, str>>>(&mut self, param: TypeDef<S>) -> &mut Self {
        self.params.push(param.to_owned());
        self
    }

    pub fn build(self) -> Parameters {
        Parameters {
            vec: Arc::new(self.params),
        }
    }
}
