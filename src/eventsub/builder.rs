use asknothingx2_util::oauth::ClientId;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    eventsub::{
        Condition, CreateEventSubscriptionsResponse, EventSubscriptionsResponse, SubscriptionType,
    },
    request::TwitchAPIRequest,
    types::{
        constants::{AFTER, EVENTSUB, STATUS, SUBSCRIPTIONS, SUBSCRIPTION_ID, TYPE, USER_ID},
        BroadcasterId, CampaignId, CategoryId, ConduitId, ExtensionClientId, ModeratorId,
        OrganizationId, RewardId, SessionId, Status, SubscriptionId, UserId,
    },
    TwitchAPI,
};

define_request_builder! {
    #[derive(Debug)]
    GetEventSubBuilder<'a> {
        status: &'a Status [key = STATUS, convert = as_ref],
        kind: &'a SubscriptionType [key = TYPE, convert = as_ref],
        user_id: &'a UserId [key = USER_ID],
        subscription_id: &'a SubscriptionId [key = SUBSCRIPTION_ID],
        after: &'a str [key = AFTER],
    } -> EventSubscriptionsResponse;
    endpoint_type: GetEventSub,
    method: GET,
    path: [EVENTSUB, SUBSCRIPTIONS],

}

#[derive(Debug, Clone, Serialize)]
pub struct CreateEventSubBuilder<'a> {
    #[serde(skip)]
    api: &'a TwitchAPI,
    #[serde(rename = "type")]
    kind: SubscriptionType,
    version: String,
    condition: Condition,
    transport: TransportRequest,
}

impl<'a> CreateEventSubBuilder<'a> {
    pub fn webhook(
        api: &'a TwitchAPI,
        kind: SubscriptionType,
        callback: Url,
        secret: String,
    ) -> Self {
        let version = kind.version().to_string();
        Self {
            api,
            kind,
            version,
            condition: Condition::default(),
            transport: TransportRequest::Webhook { callback, secret },
        }
    }

    pub fn websocket(api: &'a TwitchAPI, kind: SubscriptionType, session_id: SessionId) -> Self {
        let version = kind.version().to_string();
        Self {
            api,
            version,
            kind,
            condition: Condition::default(),
            transport: TransportRequest::Websocket { session_id },
        }
    }

    pub fn conduit(api: &'a TwitchAPI, kind: SubscriptionType, conduit_id: ConduitId) -> Self {
        let version = kind.version().to_string();
        Self {
            api,
            kind,
            version,
            condition: Condition::default(),
            transport: TransportRequest::Conduit { conduit_id },
        }
    }

    pub fn broadcaster_user_id(mut self, value: BroadcasterId) -> Self {
        self.condition = self.condition.broadcaster_user_id(value);
        self
    }
    pub fn moderator_user_id(mut self, value: ModeratorId) -> Self {
        self.condition = self.condition.moderator_user_id(value);
        self
    }
    pub fn broadcaster_id(mut self, value: BroadcasterId) -> Self {
        self.condition = self.condition.broadcaster_id(value);
        self
    }
    pub fn user_id(mut self, value: UserId) -> Self {
        self.condition = self.condition.user_id(value);
        self
    }
    pub fn client_id(mut self, value: ClientId) -> Self {
        self.condition = self.condition.client_id(value);
        self
    }
    pub fn from_broadcaster_user_id(mut self, value: BroadcasterId) -> Self {
        self.condition = self.condition.from_broadcaster_user_id(value);
        self
    }
    pub fn to_broadcaster_user_id(mut self, value: BroadcasterId) -> Self {
        self.condition = self.condition.to_broadcaster_user_id(value);
        self
    }
    pub fn reward_id(mut self, value: RewardId) -> Self {
        self.condition = self.condition.reward_id(value);
        self
    }
    pub fn conduit_id(mut self, value: ConduitId) -> Self {
        self.condition = self.condition.conduit_id(value);
        self
    }
    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.condition = self.condition.organization_id(value);
        self
    }
    pub fn category_id(mut self, value: CategoryId) -> Self {
        self.condition = self.condition.category_id(value);
        self
    }
    pub fn campaign_id(mut self, value: CampaignId) -> Self {
        self.condition = self.condition.campaign_id(value);
        self
    }
    pub fn extension_client_id(mut self, value: ExtensionClientId) -> Self {
        self.condition = self.condition.extension_client_id(value);
        self
    }

    pub fn build(self) -> TwitchAPIRequest<CreateEventSubscriptionsResponse> {
        let mut url = self.api.build_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[EVENTSUB, SUBSCRIPTIONS]);

        let body = serde_json::to_string(&self).ok();

        TwitchAPIRequest::new(
            crate::request::EndpointType::CreateEventSub,
            url,
            reqwest::Method::POST,
            self.api.header_json(),
            body,
            self.api.client.clone(),
        )
    }

    pub async fn send(self) -> Result<reqwest::Response, crate::Error> {
        self.build().send().await
    }

    pub async fn json(self) -> Result<CreateEventSubscriptionsResponse, crate::Error> {
        self.build().json().await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "lowercase")]
pub enum TransportRequest {
    Webhook { callback: Url, secret: String },
    Websocket { session_id: SessionId },
    Conduit { conduit_id: ConduitId },
}
