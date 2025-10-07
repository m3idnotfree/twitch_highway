use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCheer {
    pub is_anonymous: bool,
    pub user_id: Option<UserId>,
    pub user_login: Option<String>,
    pub user_name: Option<String>,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub message: String,
    pub bits: u64,
}
