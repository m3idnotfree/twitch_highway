use serde::{Deserialize, Serialize};

use crate::ads::{AdSchedule, SnoozeNextAd, StartCommercial};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCommercialResponse {
    pub data: Vec<StartCommercial>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdScheduleResponse {
    pub data: Vec<AdSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnoozeNextAdResponse {
    pub data: Vec<SnoozeNextAd>,
}
