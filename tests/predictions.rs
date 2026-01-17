#![cfg(feature = "predictions")]

#[macro_use]
mod common;

use twitch_highway::{
    predictions::{PredictionStatus, PredictionsAPI},
    types::{BroadcasterId, PredictionId, Title},
};

api_test!(build
    get_predictions | api | {
        api.get_predictions(&BroadcasterId::from("55696719"))
            .ids(&[PredictionId::from("d6676d5c-c86e-44d2-bfc4-100fb48f0656")])
    }
);
api_test!(
    create_prediction,
    [
        &BroadcasterId::from("141981764"),
        "Any leeks in the stream?",
        &[
            Title::new("Yes, give it time."),
            Title::new("Yes, give it time.")
        ],
        120
    ]
);
api_test!(build
    end_prediction |api| {
        api.end_prediction(
            &BroadcasterId::from("141981764"),
            &PredictionId::from("bc637af0-7766-4525-9308-4112f4cbf178"),
            PredictionStatus::RESOLVED,
        )
        .winning_outcome_id("73085848-a94d-4040-9d21-2cb7a89374b7")
    }
);
