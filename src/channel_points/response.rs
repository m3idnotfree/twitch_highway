use serde::Deserialize;

use crate::channel_points::{CustomReward, CustomRewardsRedemption};

#[derive(Debug, Clone, Deserialize)]
pub struct CustomRewardsResponse {
    pub data: Option<Vec<CustomReward>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CustomRewardsRedemptionResponse {
    pub data: Vec<CustomRewardsRedemption>,
}
