use asknothingx2_util::serde::{deserialize_empty_array_as_none, serialize_none_as_empty_array};

use serde::{Deserialize, Serialize};

use super::types::{BroadcasterTeam, Team};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelTeamsResponse {
    pub data: Vec<BroadcasterTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsResponse {
    #[serde(
        default,
        serialize_with = "serialize_none_as_empty_array",
        deserialize_with = "deserialize_empty_array_as_none"
    )]
    pub data: Option<Vec<Team>>,
}
