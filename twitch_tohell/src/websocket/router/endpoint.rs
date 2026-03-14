use std::{
    convert::Infallible,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use tower::{Layer, Service};

use crate::websocket::{
    IntoResponse, Request,
    router::{
        Handler,
        handler::BoxedHandler,
        route::{Route, RouteFuture},
    },
};

pub enum Endpoint<S = (), E = Infallible> {
    Route(Route<E>),
    BoxedHandler(BoxedHandler<S, E>),
}

impl<S, E> Endpoint<S, E>
where
    S: Clone + Send + Sync + 'static,
    E: 'static,
{
    pub fn with_state<S2>(self, state: S) -> Endpoint<S2, E> {
        match self {
            Endpoint::Route(route) => Endpoint::Route(route),
            Endpoint::BoxedHandler(handler) => Endpoint::Route(handler.into_route(state)),
        }
    }

    pub fn layer<L, E2>(self, layer: L) -> Endpoint<S, E2>
    where
        L: Layer<Route<E>> + Clone + Send + Sync + 'static,
        L::Service: Service<Request> + Clone + Send + Sync + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<E2> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
        E2: 'static,
    {
        match self {
            Endpoint::Route(route) => Endpoint::Route(route.layer(layer)),
            Endpoint::BoxedHandler(handler) => {
                let mapped_handler = handler.map(move |route| route.layer(layer));
                Endpoint::BoxedHandler(mapped_handler)
            }
        }
    }

    pub fn call_with_state(&self, req: Request, state: S) -> RouteFuture<E> {
        match self {
            Endpoint::Route(route) => route.clone().oneshot_owned(req),
            Endpoint::BoxedHandler(handler) => {
                let route = handler.clone().into_route(state);
                route.oneshot_owned(req)
            }
        }
    }
}

impl<S> Endpoint<S, Infallible>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        Endpoint::BoxedHandler(BoxedHandler::from_handler(handler))
    }
}

impl<S> Clone for Endpoint<S> {
    fn clone(&self) -> Self {
        match self {
            Self::Route(route) => Self::Route(route.clone()),
            Self::BoxedHandler(handler) => Self::BoxedHandler(handler.clone()),
        }
    }
}

impl<E> Debug for Endpoint<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Route(route) => f
                .debug_tuple(stringify!(Endpoint::Route))
                .field(route)
                .finish(),
            Self::BoxedHandler(handler) => f
                .debug_tuple(stringify!(Endpoint::BoxedHandler))
                .field(handler)
                .finish(),
        }
    }
}
