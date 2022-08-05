mod channel;

use dale::boxed::BoxFuture;

use crate::builder::ValueService;

pub trait Instance<C>: ValueService<C> {}

pub trait Transport<C> {
    type Serve;
    type Connect;
    type Instance: Instance<C>;

    fn connect(&self, connect: Self::Connect) -> Self::Instance;

    fn serve<S: ValueService<C> + Send + Sync + 'static>(
        self,
        service: S,
        serve: Self::Serve,
    ) -> BoxFuture<'static, ()>;
}
