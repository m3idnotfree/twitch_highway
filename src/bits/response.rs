use asknothingx2_util::serde::deserialize_empty_object_as_none;
use serde::Deserialize;

use crate::types::{DateRange, Pagination};

use super::types::{BitsLeaderboard, Cheermotes, ExtensionTransaction};

#[derive(Debug, Deserialize)]
pub struct BitsLeaderboardResponse {
    pub data: Vec<BitsLeaderboard>,
    pub date_range: DateRange,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionTransactionsResponse {
    pub data: Vec<ExtensionTransaction>,
    #[serde(default, deserialize_with = "deserialize_empty_object_as_none")]
    pub pagination: Option<Pagination>,
}
#[derive(Debug, Deserialize)]
pub struct CheermotesResponse {
    pub data: Vec<Cheermotes>,
}
