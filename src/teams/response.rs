use serde::{Deserialize, Serialize};

use super::types::{BroadcasterTeam, Team};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelTeamsResponse {
    pub data: Vec<BroadcasterTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsResponse {
    pub data: Vec<Team>,
}
