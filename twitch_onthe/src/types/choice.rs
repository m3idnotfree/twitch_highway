use serde::{Deserialize, Serialize};

use crate::ids::ChoiceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub id: ChoiceId,
    pub title: String,
    pub votes: u64,
    pub channel_points_votes: u64,
    /// Not used, will be set to 0.
    pub bits_votes: u64,
}
