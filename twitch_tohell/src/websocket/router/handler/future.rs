use std::{
    future::{self, Future, Ready},
    pin::Pin,
    task::{self, Context, Poll},
};

use pin_project_lite::pin_project;

use crate::websocket::{IntoResponse, Response};

pin_project! {
    #[project = HandlerFutureProj]
    pub enum HandlerFuture<F> {
        Future {
            #[pin]
            future: F,
        },
        Ready {
            #[pin]
            future: Ready<Response>,
        },
    }
}

impl<F> HandlerFuture<F> {
    pub fn new(future: F) -> Self {
        Self::Future { future }
    }

    pub fn from_response(response: Response) -> Self {
        Self::Ready {
            future: future::ready(response),
        }
    }
}

impl<F, Res> Future for HandlerFuture<F>
where
    F: Future<Output = Res>,
    Res: IntoResponse,
{
    type Output = Response;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            HandlerFutureProj::Future { future } => {
                let res = task::ready!(future.poll(cx));
                Poll::Ready(res.into_response())
            }
            HandlerFutureProj::Ready { future } => future.poll(cx),
        }
    }
}
