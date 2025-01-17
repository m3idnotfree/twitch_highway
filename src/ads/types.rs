use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartCommercial {
    pub length: u64,
    pub message: String,
    pub retry_after: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdSchedule {
    pub snooze_count: String,
    pub snooze_refresh_at: Option<DateTime<FixedOffset>>,
    pub next_ad_at: Option<DateTime<FixedOffset>>,
    pub duration: String,
    pub last_ad_at: Option<DateTime<FixedOffset>>,
    pub preroll_free_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnoozeNextAd {
    pub snooze_count: String,
    pub snooze_refresh_at: DateTime<FixedOffset>,
    pub next_ad_at: DateTime<FixedOffset>,
}
