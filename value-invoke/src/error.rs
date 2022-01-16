use value::de::DeserializerError;
use value_validate::Error as ValidationError;

#[derive(Debug)]
pub enum Error {
    Command { command: String, error: ActionError },
    CommandNotFound { command: String },
    Serialize(DeserializerError),
}

#[derive(Debug)]
pub enum ActionError {
    Validation(ValidationError),
}
