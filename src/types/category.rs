use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_art_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igdb_id: Option<String>,
}
