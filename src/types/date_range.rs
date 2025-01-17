use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    pub started_at: String,
    pub ended_at: String,
}
