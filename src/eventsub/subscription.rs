use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{
    eventsub::{condition::Condition, subscription_types::SubscriptionType},
    types::SubscriptionId,
};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#subscription>
#[derive(Debug, Serialize)]
pub struct Subscription {
    pub id: SubscriptionId,
    #[serde(rename = "type")]
    pub kind: SubscriptionType,
    pub version: String,
    pub status: Status,
    pub cost: u64,
    pub condition: Condition,
    pub created_at: DateTime<FixedOffset>,
}

impl<'de> serde::Deserialize<'de> for Subscription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Helper {
            id: SubscriptionId,
            status: Status,
            #[serde(rename = "type")]
            kind: SubscriptionType,
            version: String,
            condition: Condition,
            created_at: DateTime<FixedOffset>,
            cost: u64,
        }

        let helper = Helper::deserialize(deserializer)?;

        let kind = match helper.kind {
            kind @ SubscriptionType::AutomodMessageHold => {
                if helper.version == "2" {
                    SubscriptionType::AutomodMessageHoldV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionType::AutomodMessageUpdate => {
                if helper.version == "2" {
                    SubscriptionType::AutomodMessageUpdateV2
                } else {
                    kind
                }
            }
            kind @ SubscriptionType::ChannelModerate => {
                if helper.version == "2" {
                    SubscriptionType::ChannelModerateV2
                } else {
                    kind
                }
            }
            _ => helper.kind,
        };

        Ok(Subscription {
            id: helper.id,
            status: helper.status,
            kind,
            version: helper.version,
            condition: helper.condition,
            created_at: helper.created_at,
            cost: helper.cost,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
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

impl Status {
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

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Status> for String {
    fn from(value: Status) -> Self {
        value.as_str().to_string()
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "enabled" => Ok(Self::Enabled),
            "webhook_callback_verification_pending" => Ok(Self::WebhookCallbackVerificationPending),
            "webhook_callback_verification_failed" => Ok(Self::WebhookCallbackVerificationFailed),
            "notification_failures_exceeded" => Ok(Self::NotificationFailuresExceeded),
            "authorization_revoked" => Ok(Self::AuthorizationRevoked),
            "moderator_removed" => Ok(Self::ModeratorRemoved),
            "user_removed" => Ok(Self::UserRemoved),
            "chat_user_banned" => Ok(Self::ChatUserBanned),
            "version_removed" => Ok(Self::VersionRemoved),
            "beta_maintenance" => Ok(Self::BetaMaintenance),
            "websocket_disconnected" => Ok(Self::WebsocketDisconnected),
            "websocket_failed_ping_pong" => Ok(Self::WebsocketFailedPingPong),
            "websocket_received_inbound_traffic" => Ok(Self::WebsocketReceivedInboundTraffic),
            "websocket_connection_unused" => Ok(Self::WebsocketConnectionUnused),
            "websocket_internal_error" => Ok(Self::WebsocketInternalError),
            "websocket_network_timeout" => Ok(Self::WebsocketNetworkTimeout),
            "websocket_network_error" => Ok(Self::WebsocketNetworkError),
            "websocket_failed_to_reconnect" => Ok(Self::WebsocketFailedToReconnect),
            _ => Err(serde::de::Error::custom(format!("unknown status: {}", s))),
        }
    }
}
