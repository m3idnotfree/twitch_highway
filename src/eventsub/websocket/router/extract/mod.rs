mod rejection;

use std::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

use crate::eventsub::{
    websocket::{
        router::extract::rejection::{MetaRejection, SessionRejection, SubscriptionRejection},
        IntoResponse, MetaData, Request, Session as SessionPayload,
    },
    SubscriptionType as SubscriptionPayload,
};

pub trait Extract<S>: Sized {
    type Rejection: IntoResponse;

    fn call(req: &Request, state: &S) -> Result<Self, Self::Rejection>;
}

#[derive(Debug, Clone)]
pub struct Meta(pub MetaData);

impl<S> Extract<S> for Meta {
    type Rejection = MetaRejection;

    fn call(req: &Request, _state: &S) -> Result<Self, Self::Rejection> {
        let scanner = req.scanner.get_metadata(&req.data);
        serde_json::from_str(scanner)
            .map_err(|e| MetaRejection::new(format!("Invalid metadata JSON: {}", e)))
            .map(Self)
    }
}

#[derive(Debug, Clone)]
pub struct Session(pub SessionPayload);

impl<S> Extract<S> for Session {
    type Rejection = SessionRejection;

    fn call(req: &Request, _state: &S) -> Result<Self, Self::Rejection> {
        let scanner = req
            .scanner
            .get_session(&req.data)
            .map_err(|_| SessionRejection::new("Session data not found in request"))?;

        serde_json::from_str(scanner)
            .map_err(|e| SessionRejection::new(format!("Invalid session JSON: {}", e)))
            .map(Self)
    }
}

#[derive(Debug, Clone)]
pub struct Subscription(pub SubscriptionPayload);

impl<S> Extract<S> for Subscription {
    type Rejection = SubscriptionRejection;

    fn call(req: &Request, _state: &S) -> Result<Self, Self::Rejection> {
        let scanner = req
            .scanner
            .get_subscription(&req.data)
            .map_err(|_| SubscriptionRejection::new("Subscription data not found in request"))?;

        serde_json::from_str(scanner)
            .map_err(|e| SubscriptionRejection::new(format!("Invalid subscription JSON: {}", e)))
            .map(Self)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct State<S>(pub S);

impl<OuterState, InnerState> Extract<OuterState> for State<InnerState>
where
    InnerState: FromRef<OuterState>,
    OuterState: Send + Sync,
{
    type Rejection = Infallible;

    fn call(_req: &Request, state: &OuterState) -> Result<Self, Self::Rejection> {
        let inner_state = InnerState::from_ref(state);
        Ok(Self(inner_state))
    }
}

impl<S> Deref for State<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for State<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait FromRef<T> {
    fn from_ref(input: &T) -> Self;
}

impl<T> FromRef<T> for T
where
    T: Clone,
{
    fn from_ref(input: &T) -> Self {
        input.clone()
    }
}
