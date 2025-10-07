use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, HypeTrainId, TopContribution};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainBegin {
    pub id: HypeTrainId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub total: u64,
    pub progress: u64,
    pub goal: u64,
    pub top_contributions: Vec<TopContribution>,
    pub level: u32,
    pub all_time_high_level: u32,
    pub all_time_high_total: u32,
    pub shared_train_participants: Vec<SharedTrainParticipant>,
    pub started_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    #[serde(rename = "type")]
    pub kind: HypeTrainType,
    pub is_shared_train: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainProgress {
    pub id: HypeTrainId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub total: u64,
    pub progress: u64,
    pub goal: u64,
    pub top_contributions: Vec<TopContribution>,
    pub level: u32,
    pub shared_train_participants: Option<Vec<SharedTrainParticipant>>,
    pub started_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    #[serde(rename = "type")]
    pub kind: HypeTrainType,
    pub is_shared_train: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainEnd {
    pub id: HypeTrainId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub total: u64,
    pub top_contributions: Vec<TopContribution>,
    pub level: u32,
    pub shared_train_participants: Option<Vec<SharedTrainParticipant>>,
    pub started_at: DateTime<FixedOffset>,
    pub cooldown_ends_at: String,
    pub ended_at: DateTime<FixedOffset>,
    #[serde(rename = "type")]
    pub kind: HypeTrainType,
    pub is_shared_train: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HypeTrainType {
    Treasure,
    GoldenKappa,
    Regular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTrainParticipant {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}
