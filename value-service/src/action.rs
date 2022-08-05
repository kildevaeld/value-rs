use dale::{IntoOutcome, Outcome, Service};
use futures_core::future::BoxFuture;
use value::Value;

use crate::{arguments::Arguments, signature::Signature};

pub type ActionCtx<T> = (T, Arguments);

pub trait Action<T>: Service<ActionCtx<T>> {
    fn signature(&self) -> &Signature;
}

pub type ActionBox<'a, T, E> = Box<
    dyn Action<
            T,
            Output = Outcome<Value, E, ActionCtx<T>>,
            Future = BoxFuture<'a, Outcome<Value, E, ActionCtx<T>>>,
        > + Send
        + Sync,
>;

struct ActionBoxImpl<A> {
    action: A,
}

impl<T, A> Service<ActionCtx<T>> for ActionBoxImpl<A>
where
    A: Service<ActionCtx<T>>,
    A::Future: 'static,
    <A::Output as IntoOutcome<ActionCtx<T>>>::Success: Into<Value>,
{
    type Output = Outcome<Value, <A::Output as IntoOutcome<ActionCtx<T>>>::Failure, ActionCtx<T>>;
    type Future = BoxFuture<'static, Self::Output>;

    fn call(&self, req: ActionCtx<T>) -> Self::Future {
        let future = self.action.call(req);
        Box::pin(async move {
            match future.await.into_outcome() {
                Outcome::Success(ret) => Outcome::Success(ret.into()),
                Outcome::Failure(err) => Outcome::Failure(err),
                Outcome::Next(ctx) => Outcome::Next(ctx),
            }
        })
    }
}

impl<T, A> Action<T> for ActionBoxImpl<A>
where
    A: Action<T>,
    A::Future: Send + 'static,
    <A::Output as IntoOutcome<ActionCtx<T>>>::Success: Into<Value>,
{
    fn signature(&self) -> &Signature {
        self.action.signature()
    }
}

pub fn action_box<T, A>(
    action: A,
) -> ActionBox<
    'static,
    T,
    <<A as Service<ActionCtx<T>>>::Output as IntoOutcome<ActionCtx<T>>>::Failure,
>
where
    A: Action<T> + Send + Sync + 'static,
    A::Future: Send + 'static,
    <A::Output as IntoOutcome<ActionCtx<T>>>::Success: Into<Value>,
{
    Box::new(ActionBoxImpl { action })
}
