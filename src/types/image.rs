use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub url_1x: String,
    pub url_2x: String,
    pub url_4x: String,
}
