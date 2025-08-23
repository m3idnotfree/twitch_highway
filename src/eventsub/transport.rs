use std::fmt::Debug;

use chrono::{DateTime, FixedOffset};
use serde::{de::DeserializeOwned, Deserialize, Serialize, Serializer};
use url::Url;

use crate::types::{ConduitId, SessionId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransportMethod {
    Webhook,
    Websocket,
    Conduit,
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#transport>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transport {
    pub method: TransportMethod,
    /// The callback URL where the notifications are sent.
    /// The URL must use the HTTPS protocol and port 443.
    /// See Processing an event.
    /// <https://dev.twitch.tv/docs/eventsub/handling-webhook-events/#processing-an-event>
    ///
    /// Specify this field only if method is set to webhook.
    /// NOTE: Redirects are not followed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<Url>,
    /// The secret used to verify the signature.
    /// The secret must be an ASCII string that’s a minimum of 10 characters long
    /// and a maximum of 100 characters long.
    /// For information about how the secret is used,
    /// see Verifying the event message.
    /// <https://dev.twitch.tv/docs/eventsub/handling-webhook-events/#verifying-the-event-message>
    ///
    /// Specify this field only if method is set to webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// An ID that identifies the WebSocket to send notifications to.
    /// When you connect to EventSub using WebSockets,
    /// the server returns the ID in the Welcome message.
    /// <https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#welcome-message>
    ///
    /// Specify this field only if method is set to websocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<SessionId>,
    /// An ID that identifies the conduit to send notifications to.
    /// When you create a conduit, the server returns the conduit ID.
    ///
    /// Specify this field only if method is set to conduit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conduit_id: Option<ConduitId>,
    /// The UTC date and time that the WebSocket connection was established.
    ///
    /// This is a response-only field that
    /// Create EventSub Subscription and
    /// <https://dev.twitch.tv/docs/api/reference/#create-eventsub-subscription>
    ///
    /// Get EventSub Subscription returns
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    ///
    /// if the method field is set to websocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<DateTime<FixedOffset>>,
    /// The UTC date and time that the WebSocket connection was lost.
    ///
    /// This is a response-only field that
    /// Get EventSub Subscription returns
    /// <https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions>
    ///
    /// if the method field is set to websocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<DateTime<FixedOffset>>,
}

impl Transport {
    pub fn websocket(session_id: SessionId) -> WebsocketTransport {
        WebsocketTransport::new(session_id)
    }

    pub fn webhook<T: Into<String>>(callback: Url, secret: T) -> WebhookTransport {
        WebhookTransport::new(callback, secret.into())
    }

    pub fn conduit(conduit_id: ConduitId) -> ConduitTransport {
        ConduitTransport::new(conduit_id)
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait TransportType: sealed::Sealed + Debug + Sized + Serialize + DeserializeOwned {
    fn method_name(&self) -> &'static str;
    fn into_json(self) -> Option<String> {
        Some(serde_json::to_string(&self).unwrap())
    }
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#transport>
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookTransport {
    pub callback: Url,
    pub secret: Option<String>,
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#transport>
#[derive(Debug, Clone, Deserialize)]
pub struct WebsocketTransport {
    pub session_id: SessionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<DateTime<FixedOffset>>,
}

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#transport>
#[derive(Debug, Clone, Deserialize)]
pub struct ConduitTransport {
    pub conduit_id: ConduitId,
}

impl sealed::Sealed for WebhookTransport {}
impl sealed::Sealed for WebsocketTransport {}
impl sealed::Sealed for ConduitTransport {}

impl TransportType for WebhookTransport {
    fn method_name(&self) -> &'static str {
        "webhook"
    }
}

impl TransportType for WebsocketTransport {
    fn method_name(&self) -> &'static str {
        "websocket"
    }
}

impl TransportType for ConduitTransport {
    fn method_name(&self) -> &'static str {
        "conduit"
    }
}

impl WebhookTransport {
    pub fn new(callback: Url, secret: impl Into<String>) -> Self {
        Self {
            callback,
            secret: Some(secret.into()),
        }
    }
}

impl WebsocketTransport {
    pub fn new(session_id: SessionId) -> Self {
        Self {
            session_id,
            connected_at: None,
            disconnected_at: None,
        }
    }

    pub fn set_connected_at(&mut self, connected_at: DateTime<FixedOffset>) {
        self.connected_at = Some(connected_at);
    }
}

impl ConduitTransport {
    pub fn new(conduit_id: ConduitId) -> Self {
        Self { conduit_id }
    }
}

impl Serialize for WebhookTransport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("WebhookTransport", 3)?;
        state.serialize_field("method", "webhook")?;
        state.serialize_field("callback", &self.callback)?;
        state.serialize_field("secret", &self.secret)?;
        state.end()
    }
}

impl Serialize for WebsocketTransport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut field_count = 1;
        if self.connected_at.is_some() {
            field_count += 1;
        }

        if self.disconnected_at.is_some() {
            field_count += 1;
        }

        let mut state = serializer.serialize_struct("WebsocketTransport", field_count)?;

        state.serialize_field("method", "websocket")?;
        state.serialize_field("session_id", &self.session_id)?;

        if let Some(connected_at) = &self.connected_at {
            state.serialize_field("connected_at", connected_at)?;
        }
        if let Some(disconnected_at) = &self.disconnected_at {
            state.serialize_field("disconnected_at", disconnected_at)?;
        }

        state.end()
    }
}

impl Serialize for ConduitTransport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("WebhookTransport", 1)?;
        state.serialize_field("method", "conduit")?;
        state.serialize_field("conduit_id", &self.conduit_id)?;
        state.end()
    }
}
