use serde::{Deserialize, Serialize};

use crate::types::{OutcomeId, TopPredictor};

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

#[cfg(test)]
mod tests {
    use crate::types::OutocmeColor;

    #[test]
    fn outcome_color_enum() {
        let colors = vec![(OutocmeColor::BLUE, "BLUE"), (OutocmeColor::PINK, "PINK")];

        for (color, expected_str) in colors {
            assert_eq!(color.as_str(), expected_str);
            assert_eq!(color.as_ref(), expected_str);

            let serialized = serde_json::to_string(&color).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: OutocmeColor = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }
}
