use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ccls {
    pub id: Id,
    pub description: String,
    pub name: String,
}
