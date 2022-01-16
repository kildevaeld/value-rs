#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod types;
pub mod validation;
pub mod validator;

pub use self::{
    error::*,
    validation::{equal, max, min, required, tuple, Validation, ValidationBox},
    validator::*,
};
