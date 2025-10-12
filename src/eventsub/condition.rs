use asknothingx2_util::oauth::ClientId;
use serde::{Deserialize, Serialize};

use crate::types::{
    BroadcasterId, CampaignId, CategoryId, ConduitId, ExtensionClientId, ModeratorId,
    OrganizationId, RewardId, UserId,
};

/// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conditions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Condition {
    #[serde(skip_serializing_if = "Option::is_none")]
    broadcaster_user_id: Option<BroadcasterId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    moderator_user_id: Option<ModeratorId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    broadcaster_id: Option<BroadcasterId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<ClientId>,

    /// Channel Raid Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    from_broadcaster_user_id: Option<BroadcasterId>,
    /// Channel Raid Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    to_broadcaster_user_id: Option<BroadcasterId>,

    // Reward
    #[serde(skip_serializing_if = "Option::is_none")]
    reward_id: Option<RewardId>,

    /// Conduit Shard Disabled Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    conduit_id: Option<ConduitId>,

    /// Drop Entitlement Grant Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_id: Option<OrganizationId>,
    /// Drop Entitlement Grant Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<CategoryId>,
    /// Drop Entitlement Grant Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    campaign_id: Option<CampaignId>,

    /// Extension Bits Transaction Create Condition
    #[serde(skip_serializing_if = "Option::is_none")]
    extension_client_id: Option<ExtensionClientId>,
}

impl Condition {
    pub fn broadcaster_user_id(mut self, value: BroadcasterId) -> Self {
        self.broadcaster_user_id = Some(value);
        self
    }
    pub fn moderator_user_id(mut self, value: ModeratorId) -> Self {
        self.moderator_user_id = Some(value);
        self
    }
    pub fn broadcaster_id(mut self, value: BroadcasterId) -> Self {
        self.broadcaster_id = Some(value);
        self
    }
    pub fn user_id(mut self, value: UserId) -> Self {
        self.user_id = Some(value);
        self
    }
    pub fn client_id(mut self, value: ClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    pub fn from_broadcaster_user_id(mut self, value: BroadcasterId) -> Self {
        self.from_broadcaster_user_id = Some(value);
        self
    }
    pub fn to_broadcaster_user_id(mut self, value: BroadcasterId) -> Self {
        self.to_broadcaster_user_id = Some(value);
        self
    }

    pub fn reward_id(mut self, value: RewardId) -> Self {
        self.reward_id = Some(value);
        self
    }

    pub fn conduit_id(mut self, value: ConduitId) -> Self {
        self.conduit_id = Some(value);
        self
    }

    pub fn organization_id(mut self, value: OrganizationId) -> Self {
        self.organization_id = Some(value);
        self
    }
    pub fn category_id(mut self, value: CategoryId) -> Self {
        self.category_id = Some(value);
        self
    }
    pub fn campaign_id(mut self, value: CampaignId) -> Self {
        self.campaign_id = Some(value);
        self
    }

    pub fn extension_client_id(mut self, value: ExtensionClientId) -> Self {
        self.extension_client_id = Some(value);
        self
    }
}
