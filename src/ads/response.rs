use serde::Deserialize;

use super::types::{AdSchedule, SnoozeNextAd, StartCommercial};

#[derive(Debug, Deserialize)]
pub struct StartCommercialResponse {
    pub data: Vec<StartCommercial>,
}

#[derive(Debug, Deserialize)]
pub struct AdScheduleResponse {
    pub data: Vec<AdSchedule>,
}

#[derive(Debug, Deserialize)]
pub struct SnoozeNextAdResponse {
    pub data: Vec<SnoozeNextAd>,
}
