use serde::Serialize;

use crate::{
    types::{BroadcasterId, Id, Title},
    IntoRequestBody,
};

use super::types::PredictionStatus;

request_struct!(
    #[derive(Serialize)]
    CreatePredictionRequest {
        required {
            broadcaster_id: BroadcasterId,
            title: String,
            outcomes: Vec<Title>,
            prediction_window:u64,
        }
    }
);
impl IntoRequestBody for CreatePredictionRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
request_struct!(
    #[derive(Serialize)]
    EndPredictionRequest {
        required {
            broadcaster_id: BroadcasterId,
            id: Id,
            status: PredictionStatus,
        },
        optional {
            winning_outcome_id:String
        }
    }
);
impl IntoRequestBody for EndPredictionRequest {
    fn as_body(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
