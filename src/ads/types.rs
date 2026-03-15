use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::types::BroadcasterId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCommercial {
    /// seconds
    pub length: u8,
    pub message: String,
    /// seconds
    pub retry_after: u64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdSchedule {
    #[serde_as(as = "DisplayFromStr")]
    pub snooze_count: u64,
    pub snooze_refresh_at: Option<DateTime<FixedOffset>>,
    pub next_ad_at: Option<DateTime<FixedOffset>>,
    #[serde_as(as = "DisplayFromStr")]
    pub duration: u64,
    pub last_ad_at: Option<DateTime<FixedOffset>>,
    #[serde_as(as = "DisplayFromStr")]
    pub preroll_free_time: u64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnoozeNextAd {
    #[serde_as(as = "DisplayFromStr")]
    pub snooze_count: u64,
    pub snooze_refresh_at: DateTime<FixedOffset>,
    pub next_ad_at: DateTime<FixedOffset>,
}

#[derive(Serialize)]
pub(crate) struct StartCommercialBody<'a> {
    pub broadcaster_id: &'a BroadcasterId,
    pub length: u8,
}
