use crate::ValidationError;
use alloc::boxed::Box;
use core::{any::Any, fmt::Debug};
use value::Value;

pub type ValidationBox = Box<dyn Validation>;

#[cfg_attr(feature = "serde", typetag::serde(tag = "type"))]
pub trait Validation: Send + Sync + Debug {
    fn as_any(&self) -> &dyn Any;
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}

pub trait ValidationExt: Validation {
    fn boxed(self) -> ValidationBox
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<V> ValidationExt for V where V: Validation {}

// /**
//  * Required
//  */
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone)]
// pub struct Required;

// #[cfg_attr(feature = "serde", typetag::serde(name = "required"))]
// impl Validation for Required {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
//         if value.is_none() {
//             return Err(ValidationError::Required);
//         }
//         Ok(())
//     }
// }

// pub fn required() -> Required {
//     Required
// }

// /**
//  *
//  * Min
//  *
//  */
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, Copy)]
// pub struct Min(usize);

// #[cfg_attr(feature = "serde", typetag::serde(name = "min"))]
// impl Validation for Min {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
//         let ret = match value {
//             Value::String(str) => str.len() >= self.0,
//             Value::Number(n) => (n.as_u64() as usize) >= self.0,
//             Value::Bytes(bs) => bs.len() >= self.0,
//             _ => false,
//         };

//         if !ret {
//             return Err(ValidationError::Compare {
//                 expected: (self.0 as u64).into(),
//                 found: value.clone(),
//                 operator: crate::Operator::Min,
//             });
//         }

//         Ok(())
//     }
// }

// pub fn min(v: usize) -> Min {
//     Min(v)
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone, Copy)]
// pub struct Max(usize);

// #[cfg_attr(feature = "serde", typetag::serde(name = "max"))]
// impl Validation for Max {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
//         let ret = match value {
//             Value::String(str) => str.len() <= self.0,
//             Value::Number(n) => (n.as_u64() as usize) <= self.0,
//             Value::Bytes(bs) => bs.len() <= self.0,
//             _ => false,
//         };

//         if !ret {
//             return Err(ValidationError::Compare {
//                 expected: (self.0 as u64).into(),
//                 found: value.clone(),
//                 operator: crate::Operator::Min,
//             });
//         }
//         Ok(())
//     }
// }

// pub fn max(v: usize) -> Max {
//     Max(v)
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug, Clone)]
// pub struct Equal(pub Value);

// #[cfg_attr(feature = "serde", typetag::serde(name = "equal"))]
// impl Validation for Equal {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
//         if &self.0 != value {
//             return Err(ValidationError::Compare {
//                 expected: self.0.clone(),
//                 found: value.clone(),
//                 operator: crate::Operator::Min,
//             });
//         }
//         Ok(())
//     }
// }

// pub fn equal<V: Into<Value>>(value: V) -> Equal {
//     Equal(value.into())
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug)]
// pub struct Tuple(pub Vec<ValidationBox>);

// #[cfg_attr(feature = "serde", typetag::serde(name = "tuple"))]
// impl Validation for Tuple {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
//         let list = match value.as_list() {
//             Some(list) => list,
//             None => {
//                 return Err(ValidationError::InvalidType {
//                     expected: ValueType::List.into(),
//                     found: value.ty().into(),
//                 })
//             }
//         };

//         if list.len() != self.0.len() {
//             panic!("not equal len");
//         }

//         let values = self.0.iter().zip(list.iter());

//         let mut errors = Vec::default();
//         for (idx, (validation, value)) in values.enumerate() {
//             if let Err(err) = validation.validate(value) {
//                 errors.push((idx, err));
//             }
//         }

//         Ok(())
//     }
// }

// pub fn tuple<V: ValidationList>(value: V) -> Tuple {
//     Tuple(value.into_list())
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug)]
// pub struct OneOf(pub Vec<ValidationBox>);

// #[cfg_attr(feature = "serde", typetag::serde(name = "one_of"))]
// impl Validation for OneOf {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, v: &Value) -> Result<(), ValidationError> {
//         let mut errors = Vec::default();

//         for val in &self.0 {
//             if let Err(err) = val.validate(v) {
//                 errors.push(err)
//             } else {
//                 return Ok(());
//             }
//         }

//         if !errors.is_empty() {
//             return Err(ValidationError::OneOf(errors));
//         }

//         Ok(())
//     }
// }

// pub fn one_of<V: ValidationList>(value: V) -> OneOf {
//     OneOf(value.into_list())
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug)]
// pub struct Item {
//     validator: Validator,
// }

// #[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
// impl Validation for Item {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, _: &Value) -> Result<(), ValidationError> {
//         Ok(())
//     }
// }

// pub fn item<V: Into<Validator>>(value: V) -> Item {
//     Item {
//         validator: value.into(),
//     }
// }

// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Debug)]
// pub struct NumberSize(pub ValueType);

// #[cfg_attr(feature = "serde", typetag::serde(name = "item"))]
// impl Validation for NumberSize {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     fn validate(&self, _: &Value) -> Result<(), ValidationError> {
//         Ok(())
//     }
// }

// pub fn number_kind(kind: ValueType) -> NumberSize {
//     NumberSize(kind)
// }
