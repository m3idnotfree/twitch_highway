use serde::Serialize;

use crate::types::{BroadcasterId, Id, Title};

use super::types::PredictionStatus;

define_request!(
    #[derive(Serialize)]
    CreatePredictionRequest<'a> {
        req: {
            broadcaster_id: BroadcasterId,
            title: &'a str,
            outcomes: &'a[Title],
            prediction_window:u64,
        };
        to_json
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
    }
);
