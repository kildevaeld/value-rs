use crate::types::{box_error, Arguments, BoxError, BoxFuture, Parameters};
use std::future::Future;
use value::Value;

pub trait Action {
    type Error;
    type Future: Future<Output = Result<Value, Self::Error>>;

    fn parameters(&self) -> &Parameters;

    fn call(&self, args: Arguments) -> Self::Future;
}

pub type ActionBox = Box<
    dyn Action<Error = BoxError, Future = BoxFuture<'static, Result<Value, BoxError>>>
        + Send
        + Sync,
>;

impl Action for ActionBox {
    type Error = BoxError;
    type Future = BoxFuture<'static, Result<Value, BoxError>>;
    fn parameters(&self) -> &Parameters {
        (&**self).parameters()
    }

    fn call(&self, args: Arguments) -> Self::Future {
        (&**self).call(args)
    }
}

struct ActionBoxImpl<A> {
    action: A,
}

impl<A> Action for ActionBoxImpl<A>
where
    A: Action,
    A::Future: Send + 'static,
    A::Error: std::error::Error + Send + Sync + 'static,
{
    type Error = BoxError;
    type Future = BoxFuture<'static, Result<Value, Self::Error>>;

    fn parameters(&self) -> &Parameters {
        self.action.parameters()
    }

    fn call(&self, args: Arguments) -> Self::Future {
        let future = self.action.call(args);
        Box::pin(async move {
            match future.await {
                Ok(ret) => Ok(ret),
                Err(err) => Err(box_error(err)),
            }
        })
    }
}

pub fn action_box<A: Action>(action: A) -> ActionBox
where
    A: 'static + Send + Sync,
    A::Future: Send,
    A::Error: std::error::Error + Send + Sync + 'static,
{
    Box::new(ActionBoxImpl { action })
}
