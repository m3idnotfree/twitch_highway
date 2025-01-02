use asknothingx2_util::serde::deserialize_empty_object_as_none;
use serde::{Deserialize, Serialize};

use crate::types::Pagination;

use super::types::EventSubSubscription;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEventResponse {
    pub data: Vec<EventSubSubscription>,
    pub total: u64,
    pub total_cost: u64,
    pub max_total_cost: u64,
    #[serde(default, deserialize_with = "deserialize_empty_object_as_none")]
    pub pagination: Option<Pagination>,
}
