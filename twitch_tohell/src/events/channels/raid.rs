use serde::{Deserialize, Serialize};

use crate::BroadcasterId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raid {
    pub from_broadcaster_user_id: BroadcasterId,
    pub from_broadcaster_user_login: String,
    pub from_broadcaster_user_name: String,
    pub to_broadcaster_user_id: BroadcasterId,
    pub to_broadcaster_user_login: String,
    pub to_broadcaster_user_name: String,
    pub viewers: u64,
}
