use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// An array of up to 10 objects that describe users who participated in a Channel Points Prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopPredictor {
    pub user_id: UserId,
    pub user_login: String,
    pub user_name: String,
    pub channel_points_won: Option<u64>,
    pub channel_points_used: u64,
}
