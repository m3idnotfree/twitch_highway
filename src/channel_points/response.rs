use serde::Deserialize;

use super::types::{CustomRewards, CustomRewardsRedemption};

#[derive(Debug, Deserialize)]
pub struct CustomRewardsResponse {
    pub data: Vec<CustomRewards>,
}


#[derive(Debug, Deserialize)]
pub struct CustomRewardsRedemptionResponse {
    pub data: Vec<CustomRewardsRedemption>,
}
