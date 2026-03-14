use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelAdBreakBegin {
    pub duration_seconds: u64,
    pub started_at: DateTime<FixedOffset>,
    pub is_automatic: bool,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub requester_user_id: UserId,
    pub requester_user_login: String,
    pub requester_user_name: String,
}
