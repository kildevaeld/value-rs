use crate::{Error, TypedValidator, Validation};
use value::{Map, Value, ValueType};

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]
pub struct ObjectValidator {
    fields: BTreeMap<String, TypedValidator>,
    addional_props: bool,
    desc: Option<String>,
}

impl ObjectValidator {
    pub fn field(mut self, name: impl ToString, validation: impl Into<TypedValidator>) -> Self {
        self.fields.insert(name.to_string(), validation.into());
        self
    }
}

impl From<ObjectValidator> for TypedValidator {
    fn from(o: ObjectValidator) -> Self {
        TypedValidator::new(ValueType::Map).and(o)
    }
}

#[cfg_attr(feature = "serde", typetag::serde(name = "object"))]
impl Validation for ObjectValidator {
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let map = match value.as_map() {
            Some(v) => v,
            None => {
                return Err(Error::InvalidType {
                    found: value.ty(),
                    expected: ValueType::Map,
                })
            }
        };

        let mut errors = Vec::default();

        for (k, v) in map.iter() {
            let validator = match self.fields.get(k.as_str()) {
                Some(v) => v,
                None => {
                    panic!("no")
                }
            };

            if let Err(err) = validator.validate(v) {
                errors.push(err);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}
