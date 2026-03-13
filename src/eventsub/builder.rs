use asknothingx2_util::oauth::ClientId;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    eventsub::{
        Condition, CreateEventSubscriptionsResponse, EventSubscriptionsResponse, SubscriptionType,
    },
    types::{
        constants::{AFTER, EVENTSUB, STATUS, SUBSCRIPTIONS, SUBSCRIPTION_ID, TYPE, USER_ID},
        BroadcasterId, CampaignId, CategoryId, ConduitId, ExtensionClientId, ModeratorId,
        OrganizationId, RewardId, SessionId, Status, SubscriptionId, UserId,
    },
    Client, Error,
};

#[derive(Debug)]
pub struct GetEventSub<'a> {
    client: &'a Client,
    status: Option<&'a Status>,
    kind: Option<&'a SubscriptionType>,
    user_id: Option<&'a UserId>,
    subscription_id: Option<&'a SubscriptionId>,
    after: Option<&'a str>,
}

impl<'a> GetEventSub<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            status: None,
            kind: None,
            user_id: None,
            subscription_id: None,
            after: None,
        }
    }

    pub fn status(mut self, value: &'a Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn kind(mut self, value: &'a SubscriptionType) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn user_id(mut self, value: &'a UserId) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: &'a SubscriptionId) -> Self {
        self.subscription_id = Some(value);
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.after = Some(value);
        self
    }

    pub async fn send(self) -> Result<EventSubscriptionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, SUBSCRIPTIONS]);

        if let Some(val) = self.status {
            url.query_pairs_mut().append_pair(STATUS, val.as_ref());
        }
        if let Some(val) = self.kind {
            url.query_pairs_mut().append_pair(TYPE, val.as_ref());
        }
        if let Some(val) = self.user_id {
            url.query_pairs_mut().append_pair(USER_ID, val);
        }
        if let Some(val) = self.subscription_id {
            url.query_pairs_mut().append_pair(SUBSCRIPTION_ID, val);
        }
        if let Some(val) = self.after {
            url.query_pairs_mut().append_pair(AFTER, val);
        }

        let req = self.client.http_client().get(url);
        self.client.json(req).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateEventSub<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(rename = "type")]
    kind: SubscriptionType,
    version: String,
    condition: Condition,
    transport: TransportType,
}

impl<'a> CreateEventSub<'a> {
    pub(crate) fn new(
        client: &'a Client,
        kind: SubscriptionType,
        transport: impl Into<TransportType>,
    ) -> Self {
        let version = kind.version().to_string();
        Self {
            client,
            kind,
            version,
            condition: Condition::default(),
            transport: transport.into(),
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

    pub async fn send(self) -> Result<CreateEventSubscriptionsResponse, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([EVENTSUB, SUBSCRIPTIONS]);

        let req = self.client.http_client().post(url).json(&self);
        self.client.json(req).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "lowercase")]
pub enum TransportType {
    WebHook { callback: Url, secret: String },
    WebSocket { session_id: SessionId },
    Conduit { conduit_id: ConduitId },
}

impl From<(Url, String)> for TransportType {
    fn from(value: (Url, String)) -> Self {
        Self::WebHook {
            callback: value.0,
            secret: value.1,
        }
    }
}

impl From<(Url, &str)> for TransportType {
    fn from(value: (Url, &str)) -> Self {
        Self::WebHook {
            callback: value.0,
            secret: value.1.to_string(),
        }
    }
}

impl From<SessionId> for TransportType {
    fn from(session_id: SessionId) -> Self {
        Self::WebSocket { session_id }
    }
}

impl From<ConduitId> for TransportType {
    fn from(conduit_id: ConduitId) -> Self {
        Self::Conduit { conduit_id }
    }
}
