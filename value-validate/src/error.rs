use std::error::Error as StdError;
use thiserror::Error as ThisError;
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("valid")]
    Valid(#[from] valid::Error),
}
