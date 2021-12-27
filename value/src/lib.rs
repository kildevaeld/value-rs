#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
mod macros;

#[cfg(feature = "serde")]
pub mod de;
#[cfg(feature = "std")]
mod from_impl;
mod index;
mod map;
mod merge;
mod number;
#[cfg(feature = "serde")]
pub mod ser;
mod typed;
mod value;

pub use self::{index::Index, map::*, merge::*, number::*, typed::*, value::*};

#[cfg(feature = "serde")]
pub use self::{de::from_value, ser::to_value};

#[cfg(feature = "std")]
pub use from_impl::{ConvertError, TryAsRef};
