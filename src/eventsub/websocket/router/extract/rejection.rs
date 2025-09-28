use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use crate::eventsub::websocket::{IntoResponse, Response};

#[derive(Debug)]
pub struct MetaRejection {
    pub reason: String,
}

impl MetaRejection {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

impl IntoResponse for MetaRejection {
    fn into_response(self) -> Response {
        Response::error("meta_extraction_failed", self.reason)
    }
}

impl Display for MetaRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to extract metadata: {}", self.reason)
    }
}

impl StdError for MetaRejection {}

#[derive(Debug)]
pub struct SessionRejection {
    pub reason: String,
}

impl SessionRejection {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

impl IntoResponse for SessionRejection {
    fn into_response(self) -> Response {
        Response::error("session_extraction_failed", self.reason)
    }
}

impl Display for SessionRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to extract session: {}", self.reason)
    }
}

impl StdError for SessionRejection {}

#[derive(Debug)]
pub struct SubscriptionRejection {
    pub reason: String,
}

impl SubscriptionRejection {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

impl IntoResponse for SubscriptionRejection {
    fn into_response(self) -> Response {
        Response::error("subscription_extraction_failed", self.reason)
    }
}

impl Display for SubscriptionRejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to extract subscription: {}", self.reason)
    }
}

impl StdError for SubscriptionRejection {}
