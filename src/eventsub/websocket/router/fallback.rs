use std::{
    convert::Infallible,
    fmt::{Debug, Formatter, Result as FmtResult},
    future::{ready, Ready},
    task::{Context, Poll},
};

use tower::Service;

use crate::eventsub::websocket::{
    router::route::{Route, RouteFuture},
    Request, Response,
};

pub struct Fallback<E = Infallible>(pub(crate) Route<E>);

impl<E> Fallback<E> {
    pub fn call_with_state<S>(self, req: Request, _state: S) -> RouteFuture<E> {
        self.0.oneshot_owned(req)
    }
}

impl<E> Debug for Fallback<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(Fallback)).finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NotFound;

impl Service<Request> for NotFound {
    type Response = Response;
    type Error = Infallible;
    type Future = Ready<Result<Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request) -> Self::Future {
        ready(Ok(Response::default()))
    }
}
