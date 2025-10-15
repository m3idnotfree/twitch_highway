use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::{DateRange, ExtensionId, GameId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAnalytic {
    pub extension_id: ExtensionId,
    #[serde(rename = "URL")]
    pub url: Url,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAnalytic {
    pub game_id: GameId,
    #[serde(rename = "URL")]
    pub url: Url,
    #[serde(rename = "type")]
    pub kind: String,
    pub date_range: DateRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum AnalyticsType {
    #[serde(rename = "overview_v2")]
    OverviewV2,
}

impl AnalyticsType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OverviewV2 => "overview_v2",
        }
    }
}

impl From<AnalyticsType> for String {
    fn from(value: AnalyticsType) -> Self {
        value.as_str().to_string()
    }
}

impl AsRef<str> for AnalyticsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
