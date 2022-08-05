use std::collections::BTreeMap;

use async_trait::async_trait;
use value::Value;
use value_types::FromValue;

use crate::{
    arguments::{Arguments, ToArguments},
    errors::{Error, IntoArgumentsError},
    Signature,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ServiceDescription<S: Ord> {
    pub name: Option<S>,
    pub methods: BTreeMap<S, Signature>,
}

#[async_trait]
pub trait Service<C> {
    type Error;
    fn description(&self) -> &ServiceDescription<String>;

    async fn call(&self, ctx: C, name: &str, args: Arguments) -> Result<Value, Self::Error>;
}

#[async_trait]
pub trait ServiceExt<C>: Service<C> {
    async fn exec<O: FromValue + 'static, A: ToArguments + Send>(
        &self,
        ctx: C,
        name: &str,
        args: A,
    ) -> Result<O, Error>
    where
        C: Send + 'static,
        A::Error: Send + Sync + std::error::Error + 'static,
        O::Error: std::error::Error + Send + Sync + 'static,
        Self::Error: std::error::Error + Send + Sync + 'static,
    {
        let args = args.to_arguments().map_err(IntoArgumentsError::convert)?;

        match self.call(ctx, name, args).await {
            Ok(ret) => O::from_value(ret).map_err(|err| Error::Unknown(Box::new(err))),
            Err(err) => Err(Error::unknown(err)),
        }
    }
}
