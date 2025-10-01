use std::{
    convert::Infallible,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use tokio_tungstenite::tungstenite::Utf8Bytes;
use url::Url;

use crate::eventsub::{
    websocket::{scanner, MessageType, Scanner},
    SubscriptionType,
};

#[derive(Debug, Clone)]
pub struct Request {
    pub message_type: MessageType,
    pub subscription_type: Option<SubscriptionType>,
    pub scanner: Scanner,
    pub data: Utf8Bytes,
}

impl Request {
    pub fn is_keepalive(&self) -> bool {
        matches!(self.message_type, MessageType::SessionKeepalive)
    }

    pub fn is_reconnect(&self) -> bool {
        matches!(self.message_type, MessageType::SessionReconnect)
    }

    pub fn get_reconnect_url(&self) -> &str {
        self.scanner.get_reconnect_url(&self.data).unwrap()
    }
}

impl FromStr for Request {
    type Err = scanner::ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanner = Scanner::new(s)?;
        Ok(Self {
            message_type: scanner.message_type,
            subscription_type: scanner.subscription_type,
            scanner,
            data: s.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    status: Status,
    pub url: Option<String>,
    pub error_type: Option<String>,
    pub error_reason: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: Status::Success,
            url: None,
            error_type: None,
            error_reason: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Success,
    Reconnect,
    NotFound,
    Error,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Status::Success => f.write_str("success"),
            Status::Reconnect => f.write_str("reconnect"),
            Status::NotFound => f.write_str("not found"),
            Status::Error => f.write_str("error"),
        }
    }
}

impl Response {
    pub fn ok() -> Self {
        Self::default()
    }

    pub fn reconnect(url: Option<impl Into<String>>) -> Self {
        Self {
            status: Status::Reconnect,
            url: url.map(Into::into),
            ..Default::default()
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: Status::NotFound,
            ..Default::default()
        }
    }

    pub fn error(error_type: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            status: Status::Error,
            error_type: Some(error_type.into()),
            error_reason: Some(reason.into()),
            ..Default::default()
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self.status, Status::Success)
    }

    pub fn is_reconnect(&self) -> bool {
        matches!(self.status, Status::Reconnect)
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self.status, Status::NotFound)
    }

    pub fn is_error(&self) -> bool {
        matches!(self.status, Status::Error)
    }
}

/// # Rules
///
/// - `()` -> Success (no data)
/// - `Url` -> Reconnect with URL
/// - `Box<dyn Error>` -> Error
pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for () {
    fn into_response(self) -> Response {
        Response::ok()
    }
}

impl IntoResponse for Url {
    fn into_response(self) -> Response {
        Response::reconnect(Some(self))
    }
}

impl<T, K> IntoResponse for Result<T, K>
where
    T: IntoResponse,
    K: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(resp) => resp.into_response(),
            Err(err) => err.into_response(),
        }
    }
}

impl<E> IntoResponse for Box<E>
where
    E: StdError + ?Sized + 'static,
{
    fn into_response(self) -> Response {
        Response::error("error", self.to_string())
    }
}

impl IntoResponse for Infallible {
    fn into_response(self) -> Response {
        Response::ok()
    }
}
