use std::collections::HashMap;

use asknothingx2_util::api::APIRequest;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use crate::types::{Pagination, SubscriptionTypes, Transport};

crate::impl_endpoint!(
/// https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions
    GetEventSub {
        status: Option<GetEventStatus>,
        kind: Option<SubscriptionTypes>,
        user_id: Option<String>,
        after: Option<String>,
    };
    new = {
        params = {},
        init = {
            status: None,
            kind: None,
            user_id: None,
            after: None,
       }
    },
    url = ["eventsub","subscriptions"],
);

impl GetEventSub {
    pub fn set_status(&mut self, status: GetEventStatus) {
        self.status = Some(status);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetEventStatus {
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

impl GetEventStatus {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventResponse {
    pub data: Vec<EventSubSubscription>,
    pub total: u64,
    pub total_cost: u64,
    pub max_total_cost: u64,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pub pagination: Option<Pagination>,
}

/// https://github.com/serde-rs/serde/issues/2362
pub fn deserialize_empty_object<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(
        untagged,
        deny_unknown_fields,
        expecting = "object, empty object or null"
    )]
    enum Helper<T> {
        Data(T),
        Empty {},
        Null,
    }
    match Helper::deserialize(deserializer) {
        Ok(Helper::Data(data)) => Ok(Some(data)),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

impl APIRequest for GetEventSub {
    crate::impl_api_request_method!(GET);
    crate::impl_api_request_header!();

    fn url(&self) -> Url {
        let mut url = self.get_url();

        if let Some(status) = &self.status {
            url.query_pairs_mut().append_pair("status", status.as_str());
        }

        if let Some(kind) = &self.kind {
            url.query_pairs_mut().append_pair("type", kind.as_str());
        }

        if let Some(user_id) = &self.user_id {
            url.query_pairs_mut().append_pair("user_id", user_id);
        }

        if let Some(after) = &self.after {
            url.query_pairs_mut().append_pair("after", after);
        }

        url
    }
}
