use std::{borrow::Cow, collections::BTreeMap, marker::PhantomData};

use crate::{
    action_box,
    arguments::{Arguments, ToArguments},
    errors::Error,
    service_ext::ServiceExt,
    Action, ActionBox, Parameters, Signature,
};
use async_trait::async_trait;
use dale::{BoxService, IntoOutcome, Outcome, ServiceExt as _};
use futures_core::future::BoxFuture;
use value::Value;
use value_types::FromValue;

#[derive(Debug, Clone)]
pub struct ServiceDescription<S> {
    pub name: Option<S>,
    pub methods: BTreeMap<S, Signature>,
}

#[async_trait]
pub trait ValueService<C> {
    fn description(&self) -> &ServiceDescription<String>;

    async fn call<O: FromValue + 'static, A: ToArguments + Send>(
        &self,
        ctx: C,
        name: &str,
        args: A,
    ) -> Result<O, Error>
    where
        O::Error: std::error::Error + Send + Sync + 'static;
}

pub struct ValueServiceBuilder<C> {
    services: BTreeMap<String, BoxService<'static, (C, Arguments), Value, Error>>,
    desc: ServiceDescription<String>,
    _c: PhantomData<C>,
}

impl<C> Default for ValueServiceBuilder<C> {
    fn default() -> Self {
        ValueServiceBuilder {
            services: BTreeMap::default(),
            desc: ServiceDescription {
                name: None,
                methods: BTreeMap::default(),
            },
            _c: PhantomData,
        }
    }
}

impl<C> ValueServiceBuilder<C> {
    pub fn add<A>(&mut self, name: &str, action: A) -> &mut Self
    where
        C: Send + 'static,
        A: Action<C> + Send + Sync + 'static + Clone,
        A::Future: Send + 'static,
        A::Output: Send,
        <A::Output as IntoOutcome<(C, Arguments)>>::Success: Into<Value> + Send,
        <A::Output as IntoOutcome<(C, Arguments)>>::Failure: Into<Error> + Send + Sync,
    {
        let signature = action.signature().clone();
        let service = action.and_then(|m| async move { m.into() }).err_into();

        self.desc.methods.insert(name.to_string(), signature);

        self.services.insert(name.to_string(), service.boxed());
        self
    }

    pub fn with<A>(mut self, name: &str, action: A) -> Self
    where
        C: Send + 'static,
        A: Action<C> + Send + Sync + 'static + Clone,
        A::Future: Send + 'static,
        A::Output: Send,
        <A::Output as IntoOutcome<(C, Arguments)>>::Success: Into<Value> + Send,
        <A::Output as IntoOutcome<(C, Arguments)>>::Failure: Into<Error> + Send + Sync,
    {
        self.add(name, action);
        self
    }
}

#[async_trait]
impl<C> ValueService<C> for ValueServiceBuilder<C>
where
    C: Send + Sync,
{
    fn description(&self) -> &ServiceDescription<String> {
        &self.desc
    }

    async fn call<O: FromValue + 'static, A: ToArguments + Send>(
        &self,
        ctx: C,
        name: &str,
        args: A,
    ) -> Result<O, Error>
    where
        O::Error: std::error::Error + Send + Sync + 'static,
    {
        let service = match self.services.get(name) {
            Some(service) => service,
            None => return Err(Error::NotFound),
        };

        service.exec(ctx, args).await
    }
}
