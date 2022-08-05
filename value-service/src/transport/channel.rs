use crate::{
    arguments::{Arguments, ToArguments},
    errors::{Error, IntoArgumentsError},
    service::{Service, ServiceDescription},
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

    fn serve<S: Service<C, Error = Error> + Send + Sync + 'static>(
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
                        let ret = service.call(ctx, &method, args).await;
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
impl<C> Service<C> for ChannelService<C>
where
    C: Send,
{
    type Error = Error;
    fn description(&self) -> &ServiceDescription<String> {
        todo!()
    }

    async fn call(&self, ctx: C, name: &str, args: Arguments) -> Result<Value, Error> {
        let (returns, wait) = oneshot::channel();

        let msg = Message::Call {
            method: name.to_string(),
            args,
            ctx,
            returns,
        };

        let mut sx = self.sender.clone();

        if let Err(err) = sx.send(msg).await {
            return Err(Error::Unknown(Box::new(err)));
        }

        wait.await.map_err(Error::unknown)?
    }
}

impl<C> Instance<C> for ChannelService<C> where C: Send {}
