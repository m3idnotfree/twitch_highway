use std::{
    convert::Infallible,
    fmt::{Debug, Formatter, Result as FmtResult},
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use pin_project_lite::pin_project;
use tower::{
    util::{BoxCloneSyncService, MapErrLayer, Oneshot},
    Layer, Service, ServiceExt,
};

use crate::eventsub::websocket::{IntoResponse, Request, Response};

pub struct Route<E = Infallible>(BoxCloneSyncService<Request, Response, E>);

impl<E> Route<E> {
    pub fn new<T>(svc: T) -> Self
    where
        T: Service<Request, Error = E> + Clone + Send + Sync + 'static,
        T::Response: IntoResponse + 'static,
        T::Future: Send + 'static,
    {
        Self(BoxCloneSyncService::new(MapIntoResponse::new(svc)))
    }

    fn oneshot_inner(&mut self, req: Request) -> RouteFuture<E> {
        RouteFuture::new(self.0.clone().oneshot(req))
    }

    pub fn oneshot_owned(self, req: Request) -> RouteFuture<E> {
        RouteFuture::new(self.0.oneshot(req))
    }

    pub fn layer<L, E2>(self, layer: L) -> Route<E2>
    where
        L: Layer<Route<E>> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + Sync + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<E2> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
        E2: 'static,
    {
        let layer = (MapErrLayer::new(Into::into), layer);

        Route::new(layer.layer(self))
    }
}

impl<E> Clone for Route<E> {
    #[track_caller]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E> Debug for Route<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(Route)).finish_non_exhaustive()
    }
}

impl<E> Service<Request> for Route<E> {
    type Response = Response;
    type Error = E;
    type Future = RouteFuture<E>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.oneshot_inner(req)
    }
}

type BoxedServiceOneShot<E> = Oneshot<BoxCloneSyncService<Request, Response, E>, Request>;

pin_project! {
    pub struct RouteFuture<E> {
        #[pin]
        future: BoxedServiceOneShot<E>,
    }
}

impl<E> RouteFuture<E> {
    fn new(future: BoxedServiceOneShot<E>) -> Self {
        Self { future }
    }
}

impl<E> Debug for RouteFuture<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(RouteFuture))
            .finish_non_exhaustive()
    }
}

impl<E> Future for RouteFuture<E> {
    type Output = Result<Response, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = ready!(self.project().future.poll(cx))?;
        Poll::Ready(Ok(res.into_response()))
    }
}

#[derive(Clone)]
pub struct MapIntoResponse<S> {
    future: S,
}

impl<S> MapIntoResponse<S> {
    pub fn new(future: S) -> Self {
        Self { future }
    }
}

impl<E> Debug for MapIntoResponse<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(MapIntoResponse))
            .finish_non_exhaustive()
    }
}

impl<S> Service<Request> for MapIntoResponse<S>
where
    S: Service<Request>,
    S::Response: IntoResponse,
{
    type Response = Response;
    type Error = S::Error;
    type Future = MapIntoResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.future.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        MapIntoResponseFuture {
            future: self.future.call(req),
        }
    }
}

pin_project! {
    pub struct MapIntoResponseFuture<F> {
        #[pin]
        future: F,
    }
}

impl<E> Debug for MapIntoResponseFuture<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct(stringify!(MapIntoResponseFuture))
            .finish_non_exhaustive()
    }
}

impl<F, T, E> Future for MapIntoResponseFuture<F>
where
    F: Future<Output = Result<T, E>>,
    T: IntoResponse,
{
    type Output = Result<Response, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = ready!(self.project().future.poll(cx))?;
        Poll::Ready(Ok(res.into_response()))
    }
}
