use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrain {
    pub id: Id,
    pub event_type: String,
    pub event_timestamp: DateTime<FixedOffset>,
    pub version: String,
    pub event_data: HypeTrainEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrainEvent {
    pub broadcaster_id: BroadcasterId,
    pub cooldown_end_time: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    pub goal: u64,
    pub id: Id,
    pub last_contribution: HypeTrainContribution,
    pub level: u64,
    pub started_at: DateTime<FixedOffset>,
    pub top_contributions: Vec<HypeTrainContribution>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrainContribution {
    pub total: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HypeTrainStatus {
    pub current: Option<Current>,
    pub all_time_high: Option<AllTimeHigh>,
    pub shared_all_time_high: Option<SharedAllTimeHigh>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub id: String,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub level: u64,
    pub total: u64,
    pub progress: u64,
    pub goal: u64,
    pub top_contributions: Vec<TopContribution>,
    pub shared_train_participants: Option<Vec<SharedTrainParticipant>>,
    pub started_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    #[serde(rename = "type")]
    pub kind: HypeTrainType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared_train: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopContribution {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    #[serde(rename = "type")]
    pub kind: ContributionType,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedTrainParticipant {
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllTimeHigh {
    pub level: u64,
    pub total: u64,
    pub achieved_at: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedAllTimeHigh {
    pub level: u64,
    pub total: u64,
    pub achieved_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContributionType {
    Bits,
    Subscription,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HypeTrainType {
    Treasure,
    #[serde(rename = "golden_kappa")]
    GoldenKappa,
    Regular,
}
