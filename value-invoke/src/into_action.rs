use std::future::Future;

use futures_core::TryFuture;
use validate::TypedValidator;
use value::{Typed, Value};
use value_validate as validate;

use crate::{
    error::Error,
    service::{Action, ActionBox},
    types::{Arguments, Parameter, Parameters},
};

//

pub struct ActionFn<F> {
    params: Parameters,
    func: F,
}

impl<F, U> Action for ActionFn<F>
where
    F: Fn(Arguments) -> U + Send + Sync,
    U: TryFuture + Send,
    U: Future<Output = Result<Value, U::Error>>,
{
    type Error = U::Error;
    type Future = U;
    fn parameters(&self) -> &crate::types::Parameters {
        &self.params
    }

    fn call(&self, args: Arguments) -> Self::Future {
        (self.func)(args)
    }
}

pub trait Func<Args> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}

pub trait IntoAction<A> {
    type Action;
    fn action(self) -> Self::Action;
}

#[cfg(feature = "serde")]
impl<F, A, U> IntoAction<A> for F
where
    F: Fn(A) -> U + Clone,
    U: TryFuture + Send,
    U: Future<Output = Result<Value, U::Error>>,
    A: Typed,
    A: for<'a> serde_lib::Deserialize<'a>,
{
    type Action = ActionBox;
    fn action(self) -> Self::Action {
        let params =
            Parameters::default().add(Parameter::new("request", TypedValidator::new(A::typed())));
        let func = self;
        ActionFn {
            params,
            func: move |args: Arguments| {
                //
                let func = func.clone();
                let arg = args.try_get("request").map_err(Error::Serialize).unwrap();
                async move {
                    //
                    func(arg).await
                }
            },
        }
    }
}

// impl<F, A> IntoAction for F where F: Func<A> {}
