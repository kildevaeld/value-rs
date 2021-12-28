use std::{collections::BTreeMap, future::Future, pin::Pin};

use crate::types::{Arguments, Parameters};

// use async_trait::async_trait;
// pub trait Command {
//     type Input;
//     type Future;
//     fn call(&self, args: Self::Input) -> Self::Future;
// }

pub trait Action: Send + Sync {
    type Future: Future + Send;
    fn call(&self, args: Arguments) -> Self::Future;
}

pub struct Command {
    action: Box<dyn Action<Future = Pin<Box<dyn Future<Output = ()> + Send>>>>,
    params: Parameters,
}

impl Command {
    pub fn call<'a>(&'a self, args: Arguments) -> impl Future + 'a {
        async move {
            //
            self.action.call(args).await
        }
    }
}

pub struct Service {
    cmds: BTreeMap<String, Command>,
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
