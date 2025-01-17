#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::types::{DateRange, ExtensionId, GameId};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionAnalytics {
    pub extension_id: ExtensionId,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameAnalytics {
    pub game_id: GameId,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}
