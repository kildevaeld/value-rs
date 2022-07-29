#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod types;
// mod validatable_impl;

mod typedef_ext;
pub mod validation;
pub mod validator;

#[cfg(feature = "derive")]
pub use value_macros::Validatable;

pub use self::{
    error::*,
    // types::Validatable,
    validation::{Validation, ValidationBox, ValidationExt},
    validations::{equal, item, max, min, number_kind, required, tuple, typed},
    validator::*,
};

pub mod validations;
