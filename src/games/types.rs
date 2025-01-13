use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Id,
    pub name: String,
    pub box_art_url: String,
    pub igdb_id: String,
}
