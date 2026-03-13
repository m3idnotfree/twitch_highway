use serde::{Deserialize, Serialize};

use crate::types::{Amount, BroadcasterId, CampaignId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityCampaign {
    pub id: CampaignId,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharityCampaignDonation {
    pub id: String,
    pub campaign_id: CampaignId,
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub amount: Amount,
}
