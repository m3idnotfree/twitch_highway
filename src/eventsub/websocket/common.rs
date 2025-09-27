use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use tokio_tungstenite::tungstenite::Utf8Bytes;

use crate::eventsub::{
    websocket::{MessageType, Scanner},
    SubscriptionType,
};

#[derive(Debug, Clone)]
pub struct Request {
    pub message_type: MessageType,
    pub subscription_type: Option<SubscriptionType>,
    pub scanner: Scanner,
    pub data: Utf8Bytes,
}

// WARN: For testing only
// must be removed before production
impl Default for Request {
    fn default() -> Self {
        let raw = r#"
{
  "metadata": {
    "message_id": "9e004721-472b-d507-8465-c7ad77872e6c",
    "message_type": "session_welcome",
    "message_timestamp": "2025-09-16T20:32:13.868394Z"
  },
  "payload": {
    "session": {
      "id": "8255e39d_d5e714f3",
      "status": "connected",
      "keepalive_timeout_seconds": 10,
      "reconnect_url": null,
      "connected_at": "2025-09-16T20:32:13.868343Z"
    }
  }
}
"#;

        Self::from(raw)
    }
}

impl From<&str> for Request {
    fn from(value: &str) -> Self {
        let scanner = Scanner::new(value).unwrap();
        Self {
            message_type: scanner.message_type,
            subscription_type: scanner.subscription_type,
            scanner,
            data: value.into(),
        }
    }
}

impl From<Utf8Bytes> for Request {
    fn from(value: Utf8Bytes) -> Self {
        let scanner = Scanner::new(&value).unwrap();
        Self {
            message_type: scanner.message_type,
            subscription_type: scanner.subscription_type,
            scanner,
            data: value,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Response {
    pub reconnect: bool,
    pub url: Option<String>,
}

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
        Response::default()
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

impl IntoResponse for anyhow::Result<()> {
    fn into_response(self) -> Response {
        Response::default()
    }
}

#[derive(Debug)]
pub enum Error {
    NotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(stringify!(Self))
    }
}

impl StdError for Error {}
