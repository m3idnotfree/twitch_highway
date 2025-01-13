use serde::{Deserialize, Serialize};

use super::types::Goal;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalsResponse {
    pub data: Vec<Goal>,
}
