use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{BroadcasterId, Id, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Id,
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: String,
    pub broadcaster_login: String,
    pub title: String,
    pub winning_outcome_id: Option<String>,
    pub outcomes: Vec<PredictionOutComes>,
    pub prediction_window: u64,
    pub status: PredictionStatus,
    pub created_at: DateTime<FixedOffset>,
    pub ended_at: Option<DateTime<FixedOffset>>,
    pub locked_at: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionOutComes {
    pub id: Id,
    pub title: String,
    pub users: u64,
    pub channel_points: u64,
    pub top_predictors: Option<Vec<TopPredictor>>,
    pub color: PredictionColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionStatus {
    ACTIVE,
    CANCELED,
    LOCKED,
    RESOLVED,
}

impl PredictionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ACTIVE => "ACTIVE",
            Self::CANCELED => "CANCELED",
            Self::LOCKED => "LOCKED",
            Self::RESOLVED => "RESOLVED",
        }
    }
}

impl AsRef<str> for PredictionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPredictor {
    pub user_id: UserId,
    pub user_name: String,
    pub user_login: String,
    pub channel_points_used: u64,
    pub channel_points_won: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionColor {
    BLUE,
    PINK,
}

impl PredictionColor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BLUE => "BLUE",
            Self::PINK => "PINK",
        }
    }
}

impl AsRef<str> for PredictionColor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prediction_status_enum() {
        let statuses = vec![
            (PredictionStatus::ACTIVE, "ACTIVE"),
            (PredictionStatus::CANCELED, "CANCELED"),
            (PredictionStatus::LOCKED, "LOCKED"),
            (PredictionStatus::RESOLVED, "RESOLVED"),
        ];

        for (status, expected_str) in statuses {
            assert_eq!(status.as_str(), expected_str);
            assert_eq!(status.as_ref(), expected_str);

            let serialized = serde_json::to_string(&status).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: PredictionStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }

    #[test]
    fn prediction_color_enum() {
        let colors = vec![
            (PredictionColor::BLUE, "BLUE"),
            (PredictionColor::PINK, "PINK"),
        ];

        for (color, expected_str) in colors {
            assert_eq!(color.as_str(), expected_str);
            assert_eq!(color.as_ref(), expected_str);

            let serialized = serde_json::to_string(&color).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: PredictionColor = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized.as_str(), expected_str);
        }
    }

    #[test]
    fn prediction_serialization() {
        use serde_json::json;

        let json_data = json!({
            "id": "test_pred_123",
            "broadcaster_id": "broadcaster456",
            "broadcaster_name": "TestStreamer",
            "broadcaster_login": "teststreamer",
            "title": "Test Prediction",
            "winning_outcome_id": null,
            "outcomes": [
                {
                    "id": "outcome1",
                    "title": "Yes",
                    "users": 10,
                    "channel_points": 5000,
                    "top_predictors": [
                        {
                            "user_id": "user123",
                            "user_name": "TopUser",
                            "user_login": "topuser",
                            "channel_points_used": 1000,
                            "channel_points_won": 2000
                        },
                    ],
                    "color": "BLUE"
                }
            ],
            "prediction_window": 600,
            "status": "ACTIVE",
            "created_at": "2024-01-15T10:30:00Z",
            "ended_at": null,
            "locked_at": null
        });

        let prediction: Prediction = serde_json::from_value(json_data).unwrap();

        assert_eq!(prediction.id.as_str(), "test_pred_123");
        assert_eq!(prediction.broadcaster_id.as_str(), "broadcaster456");
        assert_eq!(prediction.title, "Test Prediction");
        assert_eq!(prediction.outcomes.len(), 1);
        assert_eq!(prediction.prediction_window, 600);
        assert!(matches!(prediction.status, PredictionStatus::ACTIVE));
        assert!(prediction.winning_outcome_id.is_none());
        assert!(prediction.ended_at.is_none());

        let outcome = &prediction.outcomes[0];
        assert_eq!(outcome.title, "Yes");
        assert_eq!(outcome.users, 10);
        assert_eq!(outcome.channel_points, 5000);
        assert!(outcome.top_predictors.is_some());
        assert!(matches!(outcome.color, PredictionColor::BLUE));

        let top_predictor = outcome.top_predictors.as_ref().unwrap();
        assert_eq!(top_predictor[0].user_id.as_str(), "user123");
        assert_eq!(top_predictor[0].channel_points_used, 1000);
        assert_eq!(top_predictor[0].channel_points_won, 2000);
    }
}
