use asknothingx2_util::oauth::ClientId;
use serde::{Deserialize, Serialize};

use crate::types::{
    BroadcasterId, CampaignId, CategoryId, ConduitId, ExtensionClientId, ModeratorId,
    OrganizationId, RewardId, UserId,
};

define_request!(
    /// <https://dev.twitch.tv/docs/eventsub/eventsub-reference/#conditions>
    #[derive(Default, Debug, Clone, Serialize, Deserialize)]
    Condition {
        opts: {
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_user_id: BroadcasterId,
            #[serde(skip_serializing_if = "Option::is_none")]
            moderator_user_id: ModeratorId,
            #[serde(skip_serializing_if = "Option::is_none")]
            broadcaster_id: BroadcasterId,
            #[serde(skip_serializing_if = "Option::is_none")]
            user_id: UserId,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_id: ClientId,

            /// Channel Raid Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            from_broadcaster_user_id: BroadcasterId,
            /// Channel Raid Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            to_broadcaster_user_id: BroadcasterId,

            // Rewoard
            #[serde(skip_serializing_if = "Option::is_none")]
            reward_id: RewardId,

            /// Conduit Shard Disabled Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            conduit_id: ConduitId,

            /// Drop Entitlement Grant Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            organization_id: OrganizationId,
            /// Drop Entitlement Grant Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            category_id: CategoryId,
            /// Drop Entitlement Grant Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            campaign_id: CampaignId,

            /// Extension Bits Transaction Create Condition
            #[serde(skip_serializing_if = "Option::is_none")]
            extension_client_id: ExtensionClientId,
        }
    }
);
