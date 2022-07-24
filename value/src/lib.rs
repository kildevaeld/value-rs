#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
mod macros;

#[cfg(feature = "serde")]
pub mod de;
mod from_impl;
mod index;
mod map;
mod merge;
mod number;
#[cfg(feature = "serde")]
pub mod ser;
// mod typed;
mod value;

pub use self::{index::Index, map::*, merge::*, number::*, value::*};

#[cfg(feature = "serde")]
pub use self::{de::from_value, ser::to_value};

#[cfg(not(feature = "serde"))]
pub fn to_value<T: Into<Value>>(value: T) -> Value {
    value.into()
}

pub use from_impl::*;
