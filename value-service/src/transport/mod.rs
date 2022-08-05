// mod channel;

use dale::boxed::BoxFuture;

use crate::service::Service;

pub trait Instance<C>: Service<C> {}

pub trait Transport<C> {
    type Serve;
    type Connect;
    type Instance: Instance<C>;

    fn connect(&self, connect: Self::Connect) -> Self::Instance;

    fn serve<S: Service<C> + Send + Sync + 'static>(
        self,
        service: S,
        serve: Self::Serve,
    ) -> BoxFuture<'static, ()>;
}
