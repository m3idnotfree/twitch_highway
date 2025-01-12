use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::types::{BroadcasterId, ModeratorId, UserId};

#[derive(Debug, Deserialize)]
pub struct AutoModStatus {
    pub msg_id: String,
    pub is_permitted: bool,
}

#[derive(Debug, Deserialize)]
pub struct AutoModSetting {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: String,
    pub overall_level: Option<u64>,
    pub disability: u64,
    pub aggression: u64,
    pub sexuality_sex_or_gender: u64,
    pub misogyny: u64,
    pub bullying: u64,
    pub swearing: u64,
    pub race_ethnicity_or_religion: u64,
    pub sex_based_terms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BannedUser {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub expires_at: DateTime<FixedOffset>,
    pub created_at: DateTime<FixedOffset>,
    pub reason: String,
    pub moderator_id: ModeratorId,
    pub moderator_login: String,
    pub moderator_name: String,
}

#[derive(Debug, Deserialize)]
pub struct BanUser {
    pub broadcaster_id: BroadcasterId,
    pub moderator_id: ModeratorId,
    pub user_id: UserId,
    pub created_at: DateTime<FixedOffset>,
    pub end_time: Option<DateTime<FixedOffset>>,
}
