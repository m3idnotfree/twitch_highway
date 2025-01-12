#![allow(non_snake_case)]
use serde::Deserialize;

use crate::types::DateRange;

#[derive(Debug, Deserialize)]
pub struct ExtensionAnalytics {
    pub extension_id: String,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[derive(Debug, Deserialize)]
pub struct GameAnalytics {
    pub game_id: String,
    pub URL: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}
