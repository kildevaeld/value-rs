use std::{collections::BTreeMap, future::Future, pin::Pin};

use value::Value;

use crate::{
    error::Error,
    types::{Arguments, BoxError, BoxFuture, Parameters},
};

// use async_trait::async_trait;
// pub trait Command {
//     type Input;
//     type Future;
//     fn call(&self, args: Self::Input) -> Self::Future;
// }

pub trait Action: Send + Sync {
    type Error;
    type Future: Future<Output = Result<Value, Self::Error>> + Send;

    fn parameters(&self) -> &Parameters;

    fn call(&self, args: Arguments) -> Self::Future;
}

pub type ActionBox =
    Box<dyn Action<Error = BoxError, Future = BoxFuture<'static, Result<Value, BoxError>>>>;

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
            //
            match future.await {
                Ok(ret) => Ok(ret),
                Err(err) => Err(box_error(err)),
            }
        })
    }
}

fn box_error<E: std::error::Error + Send + Sync + 'static>(error: E) -> BoxError {
    Box::new(error)
}

// pub struct Command {
//     action: Box<dyn Action<Future = Pin<Box<dyn Future<Output = ()> + Send>>>>,
//     params: Parameters,
// }

// impl Command {
//     pub fn call<'a>(&'a self, args: Arguments) -> impl Future + 'a {
//         async move {
//             //

//             self.params.validate(&args)?

//             self.action.call(args).await
//         }
//     }
// }

pub struct Service {
    cmds: BTreeMap<String, ActionBox>,
}

impl Service {
    pub async fn call(&self, method: impl AsRef<str>, args: impl Into<Arguments>) {
        let cmd = match self.cmds.get(method.as_ref()) {
            Some(s) => s,
            None => panic!("not method"),
        };

        cmd.call(args.into()).await;
    }
}
