use crate::{
    arguments::{Arguments, ToArguments},
    errors::Error,
};
use async_trait::async_trait;
use dale::{IntoOutcome, Outcome, Service};
use value_types::{FromValue, IntoValue};

#[async_trait]
pub trait ServiceExt<C>: Service<(C, Arguments)> {
    async fn exec<'a, O, T>(&self, ctx: C, args: T) -> Result<O, Error>
    where
        <Self::Output as IntoOutcome<(C, Arguments)>>::Failure:
            std::error::Error + Send + Sync + 'static,
        <Self::Output as IntoOutcome<(C, Arguments)>>::Success: IntoValue,
        <<Self::Output as IntoOutcome<(C, Arguments)>>::Success as IntoValue>::Error:
            std::error::Error + Send + Sync + 'static,
        C: Send + 'a,
        T: ToArguments + Send,
        O: FromValue + 'static,
        O::Error: std::error::Error + Send + Sync + 'static,
    {
        let args = args.to_arguments().map_err(Into::into)?;

        let ret = match self.call((ctx, args)).await.into_outcome() {
            Outcome::Failure(err) => Err(Error::unknown(err)),
            Outcome::Success(ret) => {
                //
                let value = ret.into_value().map_err(Error::unknown)?;

                let ret = O::from_value(value).map_err(Error::unknown)?;

                Ok(ret)
            }
            Outcome::Next(_) => Err(Error::NotFound),
        };

        ret
    }
}

impl<C, S> ServiceExt<C> for S where S: Service<(C, Arguments)> {}
