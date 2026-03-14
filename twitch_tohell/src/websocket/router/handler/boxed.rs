use std::{
    convert::Infallible,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use crate::websocket::router::{Handler, Route};

pub struct BoxedHandler<S, E>(Box<dyn ErasedHandler<S, E>>);

impl<S> BoxedHandler<S, Infallible>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Self(Box::new(MakeErasedHandler {
            handler,
            into_route: |handler, state| Route::new(Handler::with_state(handler, state)),
        }))
    }
}

impl<S, E> BoxedHandler<S, E> {
    pub fn map<F, E2>(self, f: F) -> BoxedHandler<S, E2>
    where
        S: 'static,
        E: 'static,
        F: FnOnce(Route<E>) -> Route<E2> + Clone + Send + Sync + 'static,
        E2: 'static,
    {
        BoxedHandler(Box::new(Map {
            inner: self.0,
            layer: Box::new(f),
        }))
    }

    pub fn into_route(self, state: S) -> Route<E> {
        self.0.into_route(state)
    }
}

impl<S, E> Clone for BoxedHandler<S, E> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl<S, E> Debug for BoxedHandler<S, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple(stringify!(BoxedHandler)).finish()
    }
}

trait ErasedHandler<S, E>: Send + Sync {
    fn clone_box(&self) -> Box<dyn ErasedHandler<S, E>>;
    fn into_route(self: Box<Self>, state: S) -> Route<E>;
}

struct MakeErasedHandler<H, S> {
    handler: H,
    into_route: fn(H, S) -> Route,
}

impl<H, T> Clone for MakeErasedHandler<H, T>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            into_route: self.into_route,
        }
    }
}

impl<H, S> ErasedHandler<S, Infallible> for MakeErasedHandler<H, S>
where
    H: Clone + Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedHandler<S, Infallible>> {
        Box::new(self.clone())
    }

    fn into_route(self: Box<Self>, state: S) -> Route {
        (self.into_route)(self.handler, state)
    }
}

struct Map<S, E, E2> {
    pub inner: Box<dyn ErasedHandler<S, E>>,
    pub layer: Box<dyn LayerFn<E, E2>>,
}

impl<S, E, E2> ErasedHandler<S, E2> for Map<S, E, E2>
where
    S: 'static,
    E: 'static,
    E2: 'static,
{
    fn clone_box(&self) -> Box<dyn ErasedHandler<S, E2>> {
        Box::new(Self {
            inner: self.inner.clone_box(),
            layer: self.layer.clone_box(),
        })
    }

    fn into_route(self: Box<Self>, state: S) -> Route<E2> {
        (self.layer)(self.inner.into_route(state))
    }
}

trait LayerFn<E, E2>: FnOnce(Route<E>) -> Route<E2> + Send + Sync {
    fn clone_box(&self) -> Box<dyn LayerFn<E, E2>>;
}

impl<F, E, E2> LayerFn<E, E2> for F
where
    F: FnOnce(Route<E>) -> Route<E2> + Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn LayerFn<E, E2>> {
        Box::new(self.clone())
    }
}
