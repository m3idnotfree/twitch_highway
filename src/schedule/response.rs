use asknothingx2_util::serde::{deserialize_empty_object_as_none, serialize_none_as_empty_object};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Pagination};

use super::types::{Segment, Vacation};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleResponse {
    pub data: Schedule,
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_object",
        deserialize_with = "deserialize_empty_object_as_none"
    )]
    pub pagination: Option<Pagination>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub segments: Vec<Segment>,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub vacation: Option<Vacation>,
}
