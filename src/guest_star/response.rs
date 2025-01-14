use serde::{Deserialize, Serialize};

use super::types::{GuestStarInvite, GustStarSession, GustStarSetting};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuestStarSettingsResponse {
    pub data: Vec<GustStarSetting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GustStarSessionResponse {
    pub data: Vec<GustStarSession>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GustStarInvitesResponse {
    pub data: Vec<GuestStarInvite>,
}
