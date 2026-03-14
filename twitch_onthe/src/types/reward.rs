use serde::{Deserialize, Serialize};

use crate::ids::RewardId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: RewardId,
    pub title: String,
    pub prompt: String,
    pub cost: u64,
}
