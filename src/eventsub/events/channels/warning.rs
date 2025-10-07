use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, ModeratorId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelWarningAcknowledge {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelWarningSend {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: ModeratorId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub reason: Option<String>,
    pub chat_rules_cited: Option<Vec<String>>,
}
