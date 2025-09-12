#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::types::{DateRange, ExtensionId, GameId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAnalytic {
    pub extension_id: ExtensionId,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAnalytic {
    pub game_id: GameId,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}
