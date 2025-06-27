use serde::Serialize;

use crate::types::{BroadcasterId, Id, Title};

use super::types::PredictionStatus;

define_request!(
    #[derive(Serialize)]
    CreatePredictionRequest {
        req: {
            broadcaster_id: BroadcasterId,
            title: String,
            outcomes: Vec<Title>,
            prediction_window:u64,
        };
        into_request_body
    }
);

define_request!(
    #[derive(Serialize)]
    EndPredictionRequest {
        req: {
            broadcaster_id: BroadcasterId,
            id: Id,
            status: PredictionStatus,
        },
        opts: {
            winning_outcome_id:String
        };
        into_request_body
    }
);
