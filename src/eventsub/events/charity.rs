use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, CampaignId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityDonation {
    pub id: String,
    pub campaign_id: CampaignId,
    pub broadcaster_user_id: BroadcasterId,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
    pub amount: CharityAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityCampaignStart {
    pub id: CampaignId,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
    pub current_amount: CharityAmount,
    pub target_amount: CharityAmount,
    pub started_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityCampaignProgress {
    pub id: CampaignId,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
    pub current_amount: CharityAmount,
    pub target_amount: CharityAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityCampaignStop {
    pub id: CampaignId,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
    pub current_amount: CharityAmount,
    pub target_amount: CharityAmount,
    pub stopped_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityAmount {
    pub value: u64,
    pub decimal_places: u32,
    pub currency: String,
}
