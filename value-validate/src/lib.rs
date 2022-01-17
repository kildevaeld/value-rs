#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;
mod types;
mod validatable_impl;

pub mod validation;
pub mod validator;

#[cfg(feature = "derive")]
pub use value_macros::Validatable;

pub use self::{
    error::*,
    types::Validatable,
    validation::{
        equal, item, max, min, number_kind, required, tuple, Validation, ValidationBox,
        ValidationExt,
    },
    validator::*,
};
