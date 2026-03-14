use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSubscribe {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub tier: String,
    pub is_gift: bool,
}
