use serde::{Deserialize, Serialize};

use crate::goals::Goal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalsResponse {
    pub data: Vec<Goal>,
}
