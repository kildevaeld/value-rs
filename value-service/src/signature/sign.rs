use super::Parameters;
use std::borrow::Cow;
use value_types::{TypeDef, ValueType};

#[derive(Debug, Clone)]
pub struct Signature {
    params: Parameters,
    return_type: TypeDef<Cow<'static, str>>,
}

impl Signature {
    pub fn new(params: Parameters, return_type: TypeDef<Cow<'static, str>>) -> Signature {
        Signature {
            params,
            return_type,
        }
    }

    pub fn params(&self) -> &Parameters {
        &self.params
    }

    pub fn return_type(&self) -> &TypeDef<Cow<'static, str>> {
        &self.return_type
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            params: Parameters::default(),
            return_type: ValueType::None.into(),
        }
    }
}

pub struct Call {}
