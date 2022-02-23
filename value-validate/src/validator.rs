use crate::{
    types::ValidationList,
    validation::{self, Validation, ValidationBox},
    Error,
};
use alloc::collections::BTreeMap;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::any::Any;
use value::{Value, ValueType};

pub trait ValidatorBuilder {
    fn add_validation(&mut self, validation: ValidationBox);
}

pub trait ValidatorBuilderExt: ValidatorBuilder {
    fn and<V: Validation + 'static>(mut self, v: V) -> Self
    where
        Self: Sized,
    {
        self.add_validation(Box::new(v));
        self
    }
}

impl<V> ValidatorBuilderExt for V where V: ValidatorBuilder {}

pub trait ValidatorBuilderCommon: ValidatorBuilder + Sized {
    fn min(mut self, size: usize) -> Self {
        self.add_validation(Box::new(validation::min(size)));
        self
    }

    fn max(mut self, size: usize) -> Self {
        self.add_validation(Box::new(validation::max(size)));
        self
    }

    fn required(mut self) -> Self {
        self.add_validation(Box::new(validation::required()));
        self
    }

    fn equal(mut self, value: impl Into<Value>) -> Self {
        self.add_validation(Box::new(validation::equal(value)));
        self
    }

    fn one_of<V: ValidationList>(mut self, value: V) -> Self {
        self.add_validation(Box::new(validation::one_of(value)));
        self
    }
}

impl<V> ValidatorBuilderCommon for V where V: ValidatorBuilder {}

// pub type ValidatorBox = Box<dyn Validator + Send + Sync>;

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[derive(Debug)]
pub enum Validator {
    #[cfg_attr(feature = "serde", serde(rename = "boolean"))]
    Bool(BoolValidator),
    #[cfg_attr(feature = "serde", serde(rename = "string"))]
    String(StringValidator),
    #[cfg_attr(feature = "serde", serde(rename = "object"))]
    Object(ObjectValidator),
    #[cfg_attr(feature = "serde", serde(rename = "number"))]
    Number(NumberValidator),
    #[cfg_attr(feature = "serde", serde(rename = "list"))]
    List(ListValidator),
    #[cfg_attr(feature = "serde", serde(rename = "any"))]
    Any(AnyValidator),
}

#[cfg_attr(feature = "serde", typetag::serde(name = "typed"))]
impl Validation for Validator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        match self {
            Validator::Bool(b) => b.validate(value),
            Validator::String(s) => s.validate(value),
            Validator::Number(n) => n.validate(value),
            Validator::Object(o) => o.validate(value),
            Validator::List(l) => l.validate(value),
            Validator::Any(a) => a.validate(value),
        }
    }
}

/*
    Bool Validator
*/

pub fn bool() -> BoolValidator {
    BoolValidator::default()
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]
pub struct BoolValidator {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "validations", skip_serializing_if = "Vec::is_empty")
    )]
    vals: Vec<ValidationBox>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "description", skip_serializing_if = "Option::is_none")
    )]
    desc: Option<String>,
}

#[cfg_attr(feature = "serde", typetag::serde(name = "string"))]
impl Validation for BoolValidator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if value.ty() != ValueType::String && value.ty() != ValueType::None {
            panic!("type");
        }
        let mut errors = Vec::default();

        for v in &self.vals {
            if let Err(err) = v.validate(value) {
                errors.push(err);
            }
        }

        if !errors.is_empty() {
            return Err(Error::Multi(errors));
        }
        Ok(())
    }
}

impl ValidatorBuilder for BoolValidator {
    fn add_validation(&mut self, validation: ValidationBox) {
        self.vals.push(validation);
    }
}

impl From<BoolValidator> for Validator {
    fn from(s: BoolValidator) -> Self {
        Validator::Bool(s)
    }
}

/*
    String Validator
*/

pub fn string() -> StringValidator {
    StringValidator::default()
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]
pub struct StringValidator {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "validations", skip_serializing_if = "Vec::is_empty")
    )]
    vals: Vec<ValidationBox>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "description", skip_serializing_if = "Option::is_none")
    )]
    desc: Option<String>,
}

#[cfg_attr(feature = "serde", typetag::serde(name = "string"))]
impl Validation for StringValidator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if value.ty() != ValueType::String && value.ty() != ValueType::None {
            panic!("type");
        }
        let mut errors = Vec::default();

        for v in &self.vals {
            if let Err(err) = v.validate(value) {
                errors.push(err);
            }
        }

        if !errors.is_empty() {
            return Err(Error::Multi(errors));
        }
        Ok(())
    }
}

impl ValidatorBuilder for StringValidator {
    fn add_validation(&mut self, validation: ValidationBox) {
        self.vals.push(validation);
    }
}

impl From<StringValidator> for Validator {
    fn from(s: StringValidator) -> Self {
        Validator::String(s)
    }
}

/**
 *  Object Validator
 *  
 */

pub fn object() -> ObjectValidator {
    ObjectValidator::default()
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]
pub struct ObjectValidator {
    #[cfg_attr(feature = "serde", serde(rename = "properties"))]
    fields: BTreeMap<String, Validator>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "description", skip_serializing_if = "Option::is_none")
    )]
    desc: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "validations", skip_serializing_if = "Vec::is_empty")
    )]
    vals: Vec<ValidationBox>,
}

impl ObjectValidator {
    pub fn field(mut self, name: impl ToString, validator: impl Into<Validator>) -> Self {
        self.fields.insert(name.to_string(), validator.into());
        self
    }
}

#[cfg_attr(feature = "serde", typetag::serde(name = "object"))]
impl Validation for ObjectValidator {
    fn as_any(&self) -> &dyn Any {
        self
    }
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

        for v in &self.vals {
            if let Err(err) = v.validate(value) {
                errors.push(err);
            }
        }

        for (k, validator) in self.fields.iter() {
            let val = &map[k];

            if let Err(err) = validator.validate(val) {
                errors.push(err);
            }
        }

        for (_, _) in map.iter().filter(|(k, _)| !self.fields.contains_key(*k)) {
            // TODO: Additional properties
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Multi(errors))
        }
    }
}

impl From<ObjectValidator> for Validator {
    fn from(s: ObjectValidator) -> Self {
        Validator::Object(s)
    }
}

impl ValidatorBuilder for ObjectValidator {
    fn add_validation(&mut self, validation: ValidationBox) {
        self.vals.push(validation);
    }
}

/**
 * Number Validator
 */

pub fn number() -> NumberValidator {
    NumberValidator::default()
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]
pub struct NumberValidator {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "validations", skip_serializing_if = "Vec::is_empty")
    )]
    vals: Vec<ValidationBox>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "description", skip_serializing_if = "Option::is_none")
    )]
    desc: Option<String>,
}

#[cfg_attr(feature = "serde", typetag::serde(name = "number"))]
impl Validation for NumberValidator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if !value.is_number() && value.ty() != ValueType::None {
            return Err(Error::InvalidType {
                expected: ValueType::I32,
                found: value.ty(),
            });
        }
        let mut errors = Vec::default();

        for v in &self.vals {
            if let Err(err) = v.validate(value) {
                errors.push(err);
            }
        }

        if !errors.is_empty() {
            return Err(Error::Multi(errors));
        }
        Ok(())
    }
}

impl NumberValidator {
    pub fn kind(mut self, kind: ValueType) -> Self {
        self.add_validation(Box::new(validation::number_kind(kind)));
        self
    }
}

impl ValidatorBuilder for NumberValidator {
    fn add_validation(&mut self, validation: ValidationBox) {
        self.vals.push(validation);
    }
}

impl From<NumberValidator> for Validator {
    fn from(s: NumberValidator) -> Self {
        Validator::Number(s)
    }
}

/**
 * List Validator
 */

pub fn list() -> ListValidator {
    ListValidator::default()
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]

pub struct ListValidator {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "validations", skip_serializing_if = "Vec::is_empty")
    )]
    vals: Vec<ValidationBox>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "description", skip_serializing_if = "Option::is_none")
    )]
    desc: Option<String>,
}

#[cfg_attr(feature = "serde", typetag::serde(name = "list"))]
impl Validation for ListValidator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        if value.ty() != ValueType::List && value.ty() != ValueType::None {
            panic!("type");
        }
        let mut errors = Vec::default();

        for v in &self.vals {
            if let Err(err) = v.validate(value) {
                errors.push(err);
            }
        }

        if !errors.is_empty() {
            return Err(Error::Multi(errors));
        }
        Ok(())
    }
}

impl ValidatorBuilder for ListValidator {
    fn add_validation(&mut self, validation: ValidationBox) {
        self.vals.push(validation);
    }
}

impl From<ListValidator> for Validator {
    fn from(s: ListValidator) -> Self {
        Validator::List(s)
    }
}

/**
 *
 * Any Validator
 *
 */

pub fn any() -> AnyValidator {
    AnyValidator {
        vals: vec![
            StringValidator::default().into(),
            NumberValidator::default().into(),
            ObjectValidator::default().into(),
        ],
        desc: None,
    }
}

pub fn any_of(validators: Vec<Validator>) -> AnyValidator {
    AnyValidator {
        vals: validators,
        desc: None,
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde_lib::Serialize, serde_lib::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "serde_lib"))]
#[derive(Default, Debug)]

pub struct AnyValidator {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "validations", skip_serializing_if = "Vec::is_empty")
    )]
    vals: Vec<Validator>,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "description", skip_serializing_if = "Option::is_none")
    )]
    desc: Option<String>,
}

impl From<AnyValidator> for Validator {
    fn from(s: AnyValidator) -> Self {
        Validator::Any(s)
    }
}

#[cfg_attr(feature = "serde", typetag::serde(name = "any"))]
impl Validation for AnyValidator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn validate(&self, value: &Value) -> Result<(), Error> {
        let mut errors = Vec::default();

        for v in &self.vals {
            if let Err(err) = v.validate(value) {
                errors.push(err);
            }
        }

        if !errors.is_empty() {
            return Err(Error::Multi(errors));
        }
        Ok(())
    }
}
