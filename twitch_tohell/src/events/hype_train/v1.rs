use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{BroadcasterId, HypeTrainId, TopContribution};

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
    pub last_contribution: TopContribution,
    pub level: u32,
    pub started_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    pub is_golden_kappa_train: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainProgress {
    pub id: HypeTrainId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub level: u32,
    pub total: u64,
    pub progress: u64,
    pub goal: u64,
    pub top_contributions: Vec<TopContribution>,
    pub last_contribution: TopContribution,
    pub started_at: DateTime<FixedOffset>,
    pub expires_at: DateTime<FixedOffset>,
    pub is_golden_kappa_train: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypeTrainEnd {
    pub id: HypeTrainId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub level: u32,
    pub total: u64,
    pub top_contributions: Vec<TopContribution>,
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
    pub cooldown_ends_at: String,
    pub is_golden_kappa_train: bool,
}
