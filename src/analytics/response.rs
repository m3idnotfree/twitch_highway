use asknothingx2_util::serde::deserialize_empty_object_as_none;
use serde::Deserialize;

use crate::types::Pagination;

use super::types::{ExtensionAnalytics, GameAnalytics};

#[derive(Debug, Deserialize)]
pub struct ExtensionAnalyticsResponse {
    pub data: Vec<ExtensionAnalytics>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Deserialize)]
pub struct GameAnalyticsResponse {
    pub data: Vec<GameAnalytics>,
    #[serde(
        default,
        //serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
