use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, ModeratorId, UnbanRequestId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelUnban {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: ModeratorId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelUnbanRequestCreate {
    pub id: UnbanRequestId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub text: String,
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelUnbanRequestResolve {
    pub id: UnbanRequestId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_id: ModeratorId,
    pub moderator_login: String,
    pub moderator_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub resolution_text: String,
    pub status: UnbanRequestStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnbanRequestStatus {
    Approved,
    Canceled,
    Denied,
}
