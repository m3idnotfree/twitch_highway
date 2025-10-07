use serde::{Deserialize, Serialize};

use crate::types::RewardId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: RewardId,
    pub title: String,
    pub prompt: String,
    pub cost: u64,
}
