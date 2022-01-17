use thiserror::Error as ThisError;
use value::{de::DeserializerError, ser::SerializerError};
use value_validate::Error as ValidationError;

use crate::types::BoxError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("command returned error: ")]
    Command { command: String, error: ActionError },
    #[error("command not found")]
    CommandNotFound { command: String },
    #[error("deserialize")]
    Deserialize(#[from] DeserializerError),
    #[error("serialize")]
    Serialize(#[from] SerializerError),
}

#[derive(Debug, ThisError)]
pub enum ActionError {
    #[error("validation error")]
    Validation(#[from] ValidationError),
    #[error("execution")]
    Execution(BoxError),
}
