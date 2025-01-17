use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::{DateRange, Pagination};

use super::types::{BitsLeaderboard, Cheermotes, ExtensionTransaction};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitsLeaderboardResponse {
    pub data: Vec<BitsLeaderboard>,
    pub date_range: DateRange,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionTransactionsResponse {
    pub data: Vec<ExtensionTransaction>,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheermotesResponse {
    pub data: Vec<Cheermotes>,
}
