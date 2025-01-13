use serde::Deserialize;

use super::types::StartRaid;

#[derive(Debug, Deserialize)]
pub struct StartRaidResponse {
    pub data: Vec<StartRaid>,
}
