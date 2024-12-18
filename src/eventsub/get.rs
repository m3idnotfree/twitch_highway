use std::{collections::HashMap, sync::Arc};

use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use crate::{Pagination, SubscriptionTypes};

/// https://dev.twitch.tv/docs/api/reference/#get-eventsub-subscriptions
#[derive(Debug)]
pub struct GetEventSub {
    access_token: Arc<AccessToken>,
    client_id: Arc<ClientId>,
    status: Option<GetEventStatus>,
    kind: Option<SubscriptionTypes>,
    user_id: Option<String>,
    after: Option<String>,
}

impl GetEventSub {
    pub fn new(access_token: Arc<AccessToken>, client_id: Arc<ClientId>) -> Self {
        Self {
            access_token,
            client_id,
            status: None,
            kind: None,
            user_id: None,
            after: None,
        }
    }

    pub fn set_status(&mut self, status: GetEventStatus) {
        self.status = Some(status);
    }

    pub fn get_url(&self) -> Url {
        let mut url = Url::parse(crate::TWITCH_API_BASE).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push("eventsub")
            .push("subscriptions");
        url
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
    id: String,
    status: String,
    #[serde(rename = "type")]
    kind: String,
    version: String,
    condition: HashMap<String, String>,
    created_at: DateTime<FixedOffset>,
    transport: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventResponse {
    data: Vec<EventSubSubscription>,
    total: u64,
    total_cost: u64,
    max_total_cost: u64,
    #[serde(default, deserialize_with = "deserialize_empty_object")]
    pagination: Option<Pagination>,
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
    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("Bearer", self.access_token.secret().as_str())
            .client_id(self.client_id.as_str())
            .build()
    }

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

#[cfg(test)]
mod tests {
    use crate::{
        api_general, expect_APIRequest, expect_headers, expect_response_json, GetEventResponse,
    };

    use super::GetEventSub;

    #[test]
    fn get_eventsub() {
        let get_eventsub = api_general!(GetEventSub);

        expect_APIRequest!(
            GET,
            expect_headers!(),
            "https://api.twitch.tv/helix/eventsub/subscriptions",
            json = None,
            text = None,
            urlencoded = None,
            get_eventsub
        );
    }

    #[test]
    fn get_eventsub_response() {
        expect_response_json!("{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {}\n}",
        GetEventResponse);
        let data_pagination_none = serde_json::from_str::<GetEventResponse>("{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {}\n}").unwrap();
        assert!(data_pagination_none.pagination.is_none());

        let data_pagination_some:GetEventResponse = serde_json::from_str("{\n  \"total\": 2,\n  \"data\": [\n    {\n      \"id\": \"26b1c993-bfcf-44d9-b876-379dacafe75a\",\n      \"status\": \"enabled\",\n      \"type\": \"stream.online\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"broadcaster_user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T20:08:33.12345678Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 1\n    },\n    {\n      \"id\": \"35016908-41ff-33ce-7879-61b8dfc2ee16\",\n      \"status\": \"webhook_callback_verification_pending\",\n      \"type\": \"user.update\",\n      \"version\": \"1\",\n      \"condition\": {\n        \"user_id\": \"1234\"\n      },\n      \"created_at\": \"2020-11-10T14:32:18.730260295Z\",\n      \"transport\": {\n        \"method\": \"webhook\",\n        \"callback\": \"https://this-is-a-callback.com\"\n      },\n      \"cost\": 0\n    }\n  ],\n  \"total_cost\": 1,\n  \"max_total_cost\": 10000,\n  \"pagination\": {\n\"cursor\":\"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19\"}\n}").unwrap();
        assert_eq!(
            data_pagination_some.pagination.unwrap().cursor,
            "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19".to_string()
        );
    }
}
