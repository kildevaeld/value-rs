use std::collections::BTreeMap;

use serde::Deserialize;
use value::Value;

use crate::{
    action::{action_box, Action, ActionBox},
    into_args::IntoArguments,
    types::{BoxError, Parameters},
};

#[derive(serde::Serialize)]
pub struct Command<'a> {
    #[serde(borrow)]
    name: &'a str,
    #[serde(borrow)]
    parameters: &'a Parameters,
}

#[derive(Default)]
pub struct Service {
    cmds: BTreeMap<String, ActionBox>,
}

impl Service {
    pub fn register<A: Action>(&mut self, name: impl Into<String>, action: A)
    where
        A: 'static + Send + Sync,
        A::Future: Send,
        A::Error: std::error::Error + Send + Sync + 'static,
    {
        self.cmds.insert(name.into(), action_box(action));
    }

    pub fn register_box(&mut self, name: impl Into<String>, action: ActionBox) {
        self.cmds.insert(name.into(), action);
    }
}

impl Service {
    pub async fn call(
        &self,
        method: impl AsRef<str>,
        args: impl IntoArguments,
    ) -> Result<Value, BoxError> {
        let cmd = match self.cmds.get(method.as_ref()) {
            Some(s) => s,
            None => panic!("not method"),
        };

        cmd.call(args.into_arguments().map_err(Box::new)?).await
    }

    pub fn interface<'a>(&'a self) -> Vec<Command<'a>> {
        self.cmds
            .iter()
            .map(|(k, v)| Command {
                name: k,
                parameters: v.parameters(),
            })
            .collect()
    }
}
