use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelVIPAdd {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelVIPReomve {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}
