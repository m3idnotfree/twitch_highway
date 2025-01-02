use std::{collections::HashMap, fmt};

use asknothingx2_eventsub::twitch::Transport;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubSubscription {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub version: String,
    pub condition: HashMap<String, String>,
    pub created_at: DateTime<FixedOffset>,
    pub transport: Transport,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventStatus {
    Enabled,
    WebhookCallbackVerificationPending,
    WebhookCallbackVerificationFailed,
    NotificationFailuresExceeded,
    AuthorizationRevoked,
    ModeratorRemoved,
    UserRemoved,
    ChatUserBanned,
    VersionRemoved,
    BetaMaintenance,
    WebsocketDisconnected,
    WebsocketFailedPingPong,
    WebsocketReceivedInboundTraffic,
    WebsocketConnectionUnused,
    WebsocketInternalError,
    WebsocketNetworkTimeout,
    WebsocketNetworkError,
    WebsocketFailedToReconnect,
}

impl EventStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "enabled",
            Self::WebhookCallbackVerificationPending => "webhook_callback_verification_pending",
            Self::WebhookCallbackVerificationFailed => "webhook_callback_verification_failed",
            Self::NotificationFailuresExceeded => "notification_failures_exceeded",
            Self::AuthorizationRevoked => "authorization_revoked",
            Self::ModeratorRemoved => "moderator_removed",
            Self::UserRemoved => "user_removed",
            Self::ChatUserBanned => "chat_user_banned",
            Self::VersionRemoved => "version_removed",
            Self::BetaMaintenance => "beta_maintenance",
            Self::WebsocketDisconnected => "websocket_disconnected",
            Self::WebsocketFailedPingPong => "websocket_failed_ping_pong",
            Self::WebsocketReceivedInboundTraffic => "websocket_received_inbound_traffic",
            Self::WebsocketConnectionUnused => "websocket_connection_unused",
            Self::WebsocketInternalError => "websocket_internal_error",
            Self::WebsocketNetworkTimeout => "websocket_network_timeout",
            Self::WebsocketNetworkError => "websocket_network_error",
            Self::WebsocketFailedToReconnect => "websocket_failed_to_reconnect",
        }
    }
}

impl AsRef<str> for EventStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for EventStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
