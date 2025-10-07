use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// The top contributor for a contribution type. For example, the top contributor using BITS (by aggregate) or the top contributor using subscriptions (by count).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopContribution {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    #[serde(rename = "type")]
    pub kind: ContributionType,
    pub total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContributionType {
    Bits,
    Subscription,
    Other,
}
