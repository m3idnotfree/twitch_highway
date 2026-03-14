use serde::{Deserialize, Serialize};

use crate::{TopPredictor, ids::OutcomeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub id: OutcomeId,
    pub title: String,
    pub color: OutocmeColor,
    pub users: u64,
    pub channel_points: u64,
    pub top_predictors: Option<Vec<TopPredictor>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutocmeColor {
    BLUE,
    PINK,
}

impl OutocmeColor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BLUE => "BLUE",
            Self::PINK => "PINK",
        }
    }
}

impl AsRef<str> for OutocmeColor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
