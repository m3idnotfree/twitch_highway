use serde::{Deserialize, Serialize};

use super::types::{AdSchedule, SnoozeNextAd, StartCommercial};

#[derive(Debug, Serialize, Deserialize)]
pub struct StartCommercialResponse {
    pub data: Vec<StartCommercial>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdScheduleResponse {
    pub data: Vec<AdSchedule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnoozeNextAdResponse {
    pub data: Vec<SnoozeNextAd>,
}
