use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
}
