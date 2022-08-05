use std::{convert::Infallible, marker::PhantomData};

use async_trait::async_trait;
use dale::{IntoOutcome, Outcome, Service};
use futures_core::{future::BoxFuture, Future};
use value::Value;
use value_types::HasType;

use crate::{
    arguments::{Arguments, FromArguments},
    errors::{Error, GuardError},
    func::Func,
    Action, Signature,
};

pub trait FromContext<'r, C>: Sized + Send {
    type Error: Into<GuardError>;
    fn from_context(ctx: &C) -> Result<Self, Self::Error>;
}

impl<'r, C> FromContext<'r, C> for () {
    type Error = Infallible;
    fn from_context(_ctx: &C) -> Result<Self, Self::Error> {
        Ok(())
    }
}

pub trait FromData<T>: Sized {
    type Error;
    fn from_data(ctx: &T) -> Result<Self, Self::Error>;
}

#[async_trait]
pub trait Handler<'r, C>: Send + Sync {
    type Input: FromContext<'r, C>;
    type Args: FromArguments<'r>;
    type Output: IntoOutcome<()>;
    async fn call(&'r self, input: Self::Input, args: Self::Args) -> Self::Output;
}

pub trait HandlerExt<'r, C>: Handler<'r, C> {
    fn action(self) -> HandleAction<Self, C>
    where
        Self: Sized,
        <Self::Output as IntoOutcome<()>>::Success: HasType,
    {
        HandleAction::new(self)
    }
}

impl<'r, C, T> HandlerExt<'r, C> for T where T: Handler<'r, C> {}
//

pub struct HandleFn<F, I> {
    func: F,
    _i: PhantomData<I>,
}

impl<F: Clone, I> Clone for HandleFn<F, I> {
    fn clone(&self) -> Self {
        HandleFn {
            func: self.func.clone(),
            _i: PhantomData,
        }
    }
}

impl<F, I> HandleFn<F, I> {
    pub fn new(func: F) -> HandleFn<F, I> {
        HandleFn {
            func,
            _i: PhantomData,
        }
    }
}

#[async_trait]
impl<'a, F, I, C> Handler<'a, C> for HandleFn<F, I>
where
    F: Func<I> + Send + Sync,
    I: FromArguments<'a> + Send + Sync,
    F::Output: Future + Send + 'a,
    <F::Output as Future>::Output: IntoOutcome<()> + Send,
{
    type Input = ();
    type Args = I;
    type Output = <F::Output as Future>::Output;

    async fn call(&'a self, _input: Self::Input, args: Self::Args) -> Self::Output {
        self.func.call(args).await
    }
}

pub struct HandleAction<T, C> {
    handler: T,
    sign: Signature,
    _c: PhantomData<C>,
}

impl<T: Clone, C> Clone for HandleAction<T, C> {
    fn clone(&self) -> Self {
        HandleAction {
            handler: self.handler.clone(),
            sign: self.sign.clone(),
            _c: PhantomData,
        }
    }
}

impl<T, C> HandleAction<T, C> {
    pub fn new<'a>(handler: T) -> HandleAction<T, C>
    where
        T: Handler<'a, C>,
        <T::Output as IntoOutcome<()>>::Success: HasType,
    {
        let return_type = <T::Output as IntoOutcome<()>>::Success::typed();

        let params = <T::Args as FromArguments<'a>>::parameters();

        let sign = Signature::new(params, return_type.to_owned());

        HandleAction {
            handler,
            sign,
            _c: PhantomData,
        }
    }
}

impl<T, C> Service<(C, Arguments)> for HandleAction<T, C>
where
    T: Clone + 'static,
    C: Send + 'static,
    for<'a> T: Handler<'a, C>,
    for<'a> <<T as Handler<'a, C>>::Output as IntoOutcome<()>>::Success: Into<Value>,
    for<'a> <<T as Handler<'a, C>>::Output as IntoOutcome<()>>::Failure: Into<Error>,
{
    type Output = Outcome<Value, Error, (C, Arguments)>;

    type Future = BoxFuture<'static, Self::Output>;

    fn call(&self, req: (C, Arguments)) -> Self::Future {
        let handler = self.handler.clone();
        Box::pin(async move {
            //
            let (ctx, args) = req;
            let input = match T::Input::from_context(&ctx) {
                Ok(ret) => ret,
                Err(err) => return Outcome::Failure(Error::Guard(err.into())),
            };

            let args = match T::Args::from_arguments(&args) {
                Ok(ret) => ret,
                Err(err) => return Outcome::Failure(Error::Argument(err.into())),
            };

            let ret = match handler.call(input, args).await.into_outcome() {
                Outcome::Failure(err) => panic!(),
                Outcome::Next(_) => panic!(),
                Outcome::Success(ret) => Outcome::Success(ret.into()),
            };

            ret
        })
    }
}

impl<T, C> Action<C> for HandleAction<T, C>
where
    Self: Service<(C, Arguments)>,
{
    fn signature(&self) -> &crate::Signature {
        &self.sign
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let action: HandleAction<_, i32> =
            HandleFn::new(|arg: String, next: String| async move { 2000u64 }).action();

        println!("PARAMS: {:#?}", action.signature());
        // action.call((200, Arguments::default()));
    }
}
