use serde::Serialize;

use crate::types::{BroadcasterId, Id, Title};

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
    };
    impl_body: true
);

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
    };
    impl_body: true
);
