use std::collections::BTreeMap;
use value::Value;

use crate::{
    action::{action_box, Action, ActionBox},
    into_args::IntoArguments,
    types::Parameters,
    ActionError, Arguments, Error,
};

pub trait IntoService {
    type Service: Service;
    fn into_service(self) -> Self::Service;
}

#[async_trait::async_trait]
pub trait Service {
    fn interface(&self) -> &[Interface];
    async fn call_method(&self, name: &str, args: Arguments) -> Result<Value, Error>;
}

#[async_trait::async_trait]
pub trait ServiceExt: Service {
    async fn call<A: IntoArguments + Send>(&self, name: &str, args: A) -> Result<Value, Error> {
        self.call_method(name, args.into_arguments()?).await
    }
}

impl<S> ServiceExt for S where S: Service {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Interface {
    pub name: String,
    pub parameters: Parameters,
}

#[derive(Default)]
pub struct ServiceBuilder {
    cmds: BTreeMap<String, ActionBox>,
}

impl ServiceBuilder {
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

impl IntoService for ServiceBuilder {
    type Service = ServiceBuilderService;
    fn into_service(self) -> Self::Service {
        let i = self
            .cmds
            .iter()
            .map(|(k, v)| Interface {
                name: k.to_owned(),
                parameters: v.parameters().clone(),
            })
            .collect();

        ServiceBuilderService { cmds: self.cmds, i }
    }
}

pub struct ServiceBuilderService {
    cmds: BTreeMap<String, ActionBox>,
    i: Vec<Interface>,
}

#[async_trait::async_trait]
impl Service for ServiceBuilderService {
    async fn call_method(&self, name: &str, args: Arguments) -> Result<Value, Error> {
        let cmd = match self.cmds.get(name) {
            Some(s) => s,
            None => {
                return Err(Error::CommandNotFound {
                    command: name.to_owned(),
                })
            }
        };

        let ret = cmd
            .call(args.into_arguments().map_err(|err| Error::Command {
                command: name.to_owned(),
                error: ActionError::Execution(Box::new(err)),
            })?)
            .await
            .map_err(|err| Error::Command {
                command: name.to_owned(),
                error: ActionError::Execution(err),
            })?;

        Ok(ret)
    }

    fn interface(&self) -> &[Interface] {
        &self.i
    }
}
