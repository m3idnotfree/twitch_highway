use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StartRaid {
    pub created_at: DateTime<FixedOffset>,
    pub is_mature: bool,
}
