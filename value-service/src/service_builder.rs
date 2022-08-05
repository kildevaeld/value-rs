use crate::{
    arguments::Arguments,
    errors::Error,
    service::{Service, ServiceDescription},
    dale_ext::ServiceExt,
    Action,
};
use async_trait::async_trait;
use dale::{BoxService, IntoOutcome, ServiceExt as _};
use std::{collections::BTreeMap, marker::PhantomData};
use value::Value;

pub struct ServiceBuilder<C> {
    services: BTreeMap<String, BoxService<'static, (C, Arguments), Value, Error>>,
    desc: ServiceDescription<String>,
    _c: PhantomData<C>,
}

impl<C> Default for ServiceBuilder<C> {
    fn default() -> Self {
        ServiceBuilder {
            services: BTreeMap::default(),
            desc: ServiceDescription {
                name: None,
                methods: BTreeMap::default(),
            },
            _c: PhantomData,
        }
    }
}

impl<C> ServiceBuilder<C> {
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
impl<C> Service<C> for ServiceBuilder<C>
where
    C: Send + Sync,
{
    type Error = Error;
    fn description(&self) -> &ServiceDescription<String> {
        &self.desc
    }

    async fn call(&self, ctx: C, name: &str, args: Arguments) -> Result<Value, Error> {
        let service = match self.services.get(name) {
            Some(service) => service,
            None => return Err(Error::NotFound),
        };

        service.exec(ctx, args).await
    }
}
