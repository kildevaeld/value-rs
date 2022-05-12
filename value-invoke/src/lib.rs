mod action;
mod error;
mod into_action;
mod into_args;
mod service;
mod types;

pub use once_cell;

pub use value_macros::*;

pub use async_trait::async_trait;

pub use value;

pub use self::{
    error::*,
    into_action::IntoAction,
    into_args::IntoArguments,
    service::{Interface, IntoService, Service, ServiceBuilder, ServiceExt},
    types::{Arguments, Parameter, Parameters},
};

pub mod prelude {
    pub use super::{
        into_action::IntoAction,
        into_args::IntoArguments,
        service::{IntoService, ServiceExt},
    };
}
