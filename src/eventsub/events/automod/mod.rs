pub mod v1;
pub mod v2;

use serde::{Deserialize, Serialize};

use crate::types::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodSettingsUpdate {
    pub broadcaster_user_id: UserId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: UserId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub bullying: u32,
    pub overall_level: Option<u32>,
    pub disability: u32,
    pub misogyny: u32,
    pub race_ethnicity_or_religion: u32,
    pub sex_based_terms: u32,
    pub sexuality_sex_or_gender: u32,
    pub swearing: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomodTermsUpdate {
    pub broadcaster_user_id: UserId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub moderator_user_id: UserId,
    pub moderator_user_login: String,
    pub moderator_user_name: String,
    pub action: Action,
    pub from_automod: bool,
    pub terms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheermote {
    pub prefix: String,
    pub bits: u64,
    pub tier: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    AddPermitted,
    RemovePermitted,
    AddBlocked,
    RemoveBlocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Approved,
    Denied,
    Expired,
}
