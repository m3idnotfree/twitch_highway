use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, ModeratorId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelBan {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: ModeratorId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub reason: String,
    pub banned_at: DateTime<FixedOffset>,
    pub ends_at: Option<DateTime<FixedOffset>>,
    pub is_permanent: bool,
}
