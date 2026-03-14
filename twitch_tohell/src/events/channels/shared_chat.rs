use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, SessionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSharedChatSessionBegin {
    pub session_id: SessionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub host_broadcaster_user_id: BroadcasterId,
    pub host_broadcaster_user_login: String,
    pub host_broadcaster_user_name: String,
    pub participants: Participant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSharedChatSessionUpdate {
    pub session_id: SessionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub host_broadcaster_user_id: BroadcasterId,
    pub host_broadcaster_user_login: String,
    pub host_broadcaster_user_name: String,
    pub participants: Participant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSharedChatSessionEnd {
    pub session_id: SessionId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub host_broadcaster_user_id: BroadcasterId,
    pub host_broadcaster_user_login: String,
    pub host_broadcaster_user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}
