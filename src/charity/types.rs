use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaign {
    pub id: Id,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub charity_name: String,
    pub charity_description: String,
    pub charity_logo: String,
    pub charity_website: String,
    pub current_amount: Amount,
    pub target_amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharityCampaignDonation {
    pub id: Id,
    pub campaign_id: String,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub amount: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: u64,
    pub decimal_places: u64,
    pub currency: String,
}
