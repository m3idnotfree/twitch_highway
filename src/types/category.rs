use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::CategoryId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// category or game
    pub id: CategoryId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_art_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igdb_id: Option<String>,
}
