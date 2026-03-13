use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ccl {
    pub id: String,
    pub description: String,
    pub name: String,
}
