mod macros;

pub mod convert;
// mod response;
// mod service;

use rquickjs::{FromJs, IntoJs};
pub use value;
use value::Value;
// pub use value_invoke as invoke;

// pub use self::{response::Response, service::JsService};

#[derive(Debug, Clone, PartialEq)]
pub struct Val(Value);

impl Val {
    pub const fn new(value: Value) -> Val {
        Val(value)
    }
}

impl From<Value> for Val {
    fn from(value: Value) -> Self {
        Val(value)
    }
}

impl From<Val> for Value {
    fn from(value: Val) -> Self {
        value.0
    }
}

impl std::ops::Deref for Val {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Val {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'js> FromJs<'js> for Val {
    fn from_js(ctx: rquickjs::Ctx<'js>, value: rquickjs::Value<'js>) -> rquickjs::Result<Self> {
        Ok(Val::new(convert::from_js(ctx, value)?))
    }
}

impl<'js> IntoJs<'js> for Val {
    fn into_js(self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        convert::into_js(ctx, self.0)
    }
}
