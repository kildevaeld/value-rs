#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
mod macros;

#[cfg(feature = "serde")]
pub mod de;
mod index;
mod merge;
#[cfg(feature = "serde")]
pub mod ser;
mod value;

pub use self::{index::Index, merge::*, value::*};

#[cfg(feature = "serde")]
pub use self::{de::from_value, ser::to_value};
