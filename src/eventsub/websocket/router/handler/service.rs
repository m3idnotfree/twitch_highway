use std::{
    convert::Infallible,
    marker::PhantomData,
    task::{Context, Poll},
};

use tower::Service;

use crate::eventsub::websocket::{
    router::{service::IntoServiceFuture, Handler},
    Request, Response,
};

pub struct HandlerService<H, T, S> {
    handler: H,
    state: S,
    marker: PhantomData<fn() -> T>,
}

impl<H, T, S> HandlerService<H, T, S> {
    pub fn new(handler: H, state: S) -> Self {
        Self {
            handler,
            state,
            marker: PhantomData,
        }
    }
}

impl<H, T, S> Clone for HandlerService<H, T, S>
where
    H: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            state: self.state.clone(),
            marker: PhantomData,
        }
    }
}

impl<H, T, S> Service<Request> for HandlerService<H, T, S>
where
    H: Handler<T, S>,
    S: Clone + Send + Sync,
{
    type Response = Response;
    type Error = Infallible;
    type Future = IntoServiceFuture<H::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        use futures_util::future::FutureExt;

        let handler = self.handler.clone();
        let future = Handler::call(handler, req, self.state.clone());
        let future = future.map(Ok as _);

        IntoServiceFuture::new(future)
    }
}
