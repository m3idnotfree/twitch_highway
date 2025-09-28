mod boxed;
mod future;
mod service;

pub(crate) use boxed::BoxedHandler;

use std::future::Future;

use crate::eventsub::websocket::{
    router::{
        extract::Extract,
        handler::{future::HandlerFuture, service::HandlerService},
    },
    IntoResponse, Request, Response,
};

pub trait Handler<T, S>: Clone + Send + Sync + Sized + 'static {
    type Future: Future<Output = Response> + Send + 'static;

    fn call(self, req: Request, state: S) -> Self::Future;

    fn with_state(self, state: S) -> HandlerService<Self, T, S> {
        HandlerService::new(self, state)
    }
}

impl<F, Res, Fut, S> Handler<(), S> for F
where
    F: FnOnce() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send + 'static,
    Res: IntoResponse,
{
    type Future = HandlerFuture<Fut>;

    fn call(self, _req: Request, _state: S) -> Self::Future {
        HandlerFuture::new(self())
    }
}

macro_rules! impl_handler {
    ($($ty:ident),*) => {
        impl<F, Res, Fut, S, $($ty),*> Handler<($($ty,)*), S> for F
        where
            F: FnOnce($($ty,)*) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = Res> + Send + 'static,
            Res: IntoResponse,
            S: Clone + Send + Sync + 'static,
            $( $ty: Extract<S> + Send + Sync, )*
        {
            type Future = HandlerFuture<Fut>;

            fn call(self, req: Request, state: S)->Self::Future {
                $(
                    #[allow(non_snake_case)]
                    let $ty = match $ty::call(&req, &state) {
                        Ok(t) => t,
                        Err(rejection) => {
                            return HandlerFuture::from_response(rejection.into_response());
                        }
                    };
                )*

                HandlerFuture::new(self($($ty,)*))
            }
        }
    };
}

impl_handler![T];
impl_handler![T1, T2];
impl_handler![T1, T2, T3];
impl_handler![T1, T2, T3, T4];
impl_handler![T1, T2, T3, T4, T5];
impl_handler![T1, T2, T3, T4, T5, T6];
impl_handler![T1, T2, T3, T4, T5, T6, T7];
impl_handler![T1, T2, T3, T4, T5, T6, T7, T8];
