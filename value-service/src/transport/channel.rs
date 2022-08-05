use crate::{
    arguments::{Arguments, ToArguments},
    builder::{ServiceDescription, ValueService},
    errors::{Error, IntoArgumentsError},
};

use super::{Instance, Transport};
use async_trait::async_trait;
use futures_channel::{mpsc, oneshot};
use futures_core::future::BoxFuture;
use futures_util::{sink::SinkExt, stream::StreamExt, FutureExt};
use value::{FromValueErr, Value};
use value_types::FromValue;
enum Message<C> {
    Call {
        method: String,
        args: Arguments,
        ctx: C,
        returns: oneshot::Sender<Result<Value, Error>>,
    },
    Desc {
        returns: oneshot::Sender<ServiceDescription<String>>,
    },
}

pub struct Channel<C>(mpsc::Sender<Message<C>>);

pub struct ChannelTransport<C> {
    sender: mpsc::Sender<Message<C>>,
    reader: Option<mpsc::Receiver<Message<C>>>,
}

impl<C> ChannelTransport<C> {
    pub fn new() -> ChannelTransport<C> {
        let (sender, rx) = mpsc::channel(1);
        ChannelTransport {
            sender,
            reader: Some(rx),
        }
    }
}

impl<C> Transport<C> for ChannelTransport<C>
where
    C: Send + 'static,
{
    type Serve = ();

    type Connect = ();

    type Instance = ChannelService<C>;

    fn connect(&self, _connect: Self::Connect) -> Self::Instance {
        ChannelService {
            sender: self.sender.clone(),
        }
    }

    fn serve<S: ValueService<C> + Send + Sync + 'static>(
        mut self,
        service: S,
        _serve: Self::Serve,
    ) -> BoxFuture<'static, ()> {
        let mut chann = self.reader.take().unwrap();
        let future = async move {
            //
            while let Some(next) = chann.next().await {
                match next {
                    Message::Call {
                        method,
                        args,
                        ctx,
                        returns,
                    } => {
                        let ret = service.call::<Value, _>(ctx, &method, args).await;
                        returns.send(ret).ok();
                    }
                    Message::Desc { returns } => {
                        let desc = service.description().clone();
                        returns.send(desc).ok();
                    }
                }
            }
        };

        future.boxed()
    }
}

pub struct ChannelService<C> {
    sender: mpsc::Sender<Message<C>>,
}

#[async_trait]
impl<C> ValueService<C> for ChannelService<C>
where
    C: Send,
{
    fn description(&self) -> &ServiceDescription<String> {
        todo!()
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
        let (returns, wait) = oneshot::channel();

        let msg = Message::Call {
            method: name.to_string(),
            args: args
                .to_arguments()
                .map_err(|err| err.into() as IntoArgumentsError)?,
            ctx,
            returns,
        };

        let mut sx = self.sender.clone();

        if let Err(err) = sx.send(msg).await {
            return Err(Error::Unknown(Box::new(err)));
        }

        let ret = match wait.await.map_err(Error::unknown)? {
            Ok(ret) => O::from_value(ret).map_err(Error::unknown),
            Err(err) => Err(err),
        };

        ret
    }
}

impl<C> Instance<C> for ChannelService<C> where C: Send {}
