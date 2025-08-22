use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifter_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifter_name: Option<String>,
    pub is_gift: bool,
    pub tier: Tier,

    // get broadcaster subscriptions fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_login: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    #[serde(rename(deserialize = "1000"))]
    Tier1,
    #[serde(rename(deserialize = "2000"))]
    Tier2,
    #[serde(rename(deserialize = "3000"))]
    Tier3,
}
