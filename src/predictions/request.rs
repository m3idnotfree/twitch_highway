use serde::Serialize;

use crate::types::{BroadcasterId, Id, Title};

use super::types::PredictionStatus;

define_request!(
    #[derive(Serialize)]
    CreatePredictionRequest<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId,
            title: &'a str,
            outcomes: &'a[Title],
            prediction_window:u64,
        };
        into_json
    }
);

define_request!(
    #[derive(Serialize)]
    EndPredictionRequest<'a> {
        req: {
            broadcaster_id: &'a BroadcasterId,
            id: &'a Id,
            status: PredictionStatus,
        },
        opts: {
            winning_outcome_id: &'a str
        };
    }
);
