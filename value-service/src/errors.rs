use std::{borrow::Cow, convert::Infallible};

use thiserror::Error as ThisError;
use value::FromValueErr;
use value_types::TypeDef;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, ThisError)]
pub enum ArgumentError {
    #[error("invalid type. Expected: {expected:?}, found: {found:?}")]
    IvalidType {
        expected: TypeDef<Cow<'static, str>>,
        found: TypeDef<Cow<'static, str>>,
    },
    #[error("missing argument at index: {index:}")]
    Missing { index: usize, arity: usize },
    #[error("unknown error {0}")]
    Unknown(Box<dyn std::error::Error + Send + Sync>),
}

impl ArgumentError {
    pub fn unknown<E: std::error::Error + Send + Sync + 'static>(err: E) -> ArgumentError {
        ArgumentError::Unknown(Box::new(err))
    }
}

impl From<Infallible> for ArgumentError {
    fn from(e: Infallible) -> Self {
        ArgumentError::Unknown(Box::new(e))
    }
}

impl<'a> From<FromValueErr<'a>> for ArgumentError {
    fn from(e: FromValueErr<'a>) -> Self {
        ArgumentError::Unknown(Box::new(e.to_owned()))
    }
}

#[derive(Debug, ThisError)]
pub enum GuardError {
    #[error("infallable error: {0}")]
    Infallible(#[from] Infallible),
}

#[derive(Debug, ThisError)]
pub enum IntoArgumentsError {
    #[error("convert error: {0}")]
    Convert(BoxError),
    #[error("infallable error: {0}")]
    Infallible(#[from] Infallible),
}

impl IntoArgumentsError {
    pub fn convert<E: std::error::Error + Send + Sync + 'static>(err: E) -> IntoArgumentsError {
        IntoArgumentsError::Convert(Box::new(err))
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("argument error: {0}")]
    Argument(#[from] ArgumentError),
    #[error("arguments error: {0}")]
    IntoArguments(#[from] IntoArgumentsError),
    #[error("guard error: {0}")]
    Guard(#[from] GuardError),
    #[error("infallable error: {0}")]
    Infallible(#[from] Infallible),
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("not found")]
    NotFound,
    #[error("unknown error {0}")]
    Unknown(BoxError),
}

impl Error {
    pub fn unknown<E: std::error::Error + Send + Sync + 'static>(err: E) -> Error {
        Error::Unknown(Box::new(err))
    }
}

#[derive(Debug, ThisError)]
pub enum ValidationError {}
