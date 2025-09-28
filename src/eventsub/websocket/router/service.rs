use std::{
    convert::Infallible,
    fmt::{Debug, Formatter, Result as FmtResult},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::future::Map;
use pin_project_lite::pin_project;

use crate::eventsub::websocket::router::Response;

type ServiceResponseFuture<F> = Map<F, fn(Response) -> Result<Response, Infallible>>;

pin_project! {
    pub struct IntoServiceFuture<F> {
        #[pin]
        future: ServiceResponseFuture<F>
    }
}

impl<F> IntoServiceFuture<F> {
    pub fn new(future: ServiceResponseFuture<F>) -> Self {
        Self { future }
    }
}

impl<F> Debug for IntoServiceFuture<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(IntoServiceFuture))
            .finish_non_exhaustive()
    }
}

impl<F> Future for IntoServiceFuture<F>
where
    ServiceResponseFuture<F>: Future,
{
    type Output = <ServiceResponseFuture<F> as Future>::Output;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}
