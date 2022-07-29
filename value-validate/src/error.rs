use alloc::{borrow::Cow, vec::Vec};
use core::fmt;
use thiserror::Error as ThisError;
use value::Value;
use value_types::TypeDef;

#[derive(Debug)]
pub enum Operator {
    Min,
    Max,
    Equal,
    NotEqual,
}

#[derive(Debug, ThisError)]
pub enum ValidationError {
    #[error("required")]
    Required,
    #[error("required")]
    InvalidType {
        expected: TypeDef<Cow<'static, str>>,
        found: TypeDef<Cow<'static, str>>,
    },
    #[error("required")]
    Min { expected: usize, found: usize },
    #[error("required")]
    Compare {
        operator: Operator,
        expected: Value,
        found: Value,
    },
    #[error("required")]
    OneOf(Vec<ValidationError>),
    #[error("")]
    Multi(Vec<ValidationError>),
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("")]
    Multi(Vec<ValidationError>),
    #[error("")]
    Validation(#[from] ValidationError),
}
