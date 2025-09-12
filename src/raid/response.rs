use serde::Deserialize;

use crate::raid::StartRaid;

#[derive(Debug, Clone, Deserialize)]
pub struct StartRaidResponse {
    pub data: Vec<StartRaid>,
}
