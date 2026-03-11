#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    predictions::{PredictionStatus, PredictionsAPI},
    types::{BroadcasterId, PredictionId, Title},
};

#[tokio::test]
async fn get_predictions() {
    let suite = HttpMock::new().await;
    suite.get_predictions().await;

    let result = suite
        .api()
        .get_predictions(&BroadcasterId::from("55696719"))
        .ids(&[PredictionId::from("d6676d5c-c86e-44d2-bfc4-100fb48f0656")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_prediction() {
    let suite = HttpMock::new().await;
    suite.create_prediction().await;

    let result = suite
        .api()
        .create_prediction(
            &BroadcasterId::from("141981764"),
            "Any leeks in the stream?",
            &[
                Title::new("Yes, give it time."),
                Title::new("Yes, give it time."),
            ],
            120,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn end_prediction() {
    let suite = HttpMock::new().await;
    suite.end_prediction().await;

    let result = suite
        .api()
        .end_prediction(
            &BroadcasterId::from("141981764"),
            &PredictionId::from("bc637af0-7766-4525-9308-4112f4cbf178"),
            PredictionStatus::RESOLVED,
        )
        .winning_outcome_id("73085848-a94d-4040-9d21-2cb7a89374b7")
        .send()
        .await;

    assert!(result.is_ok());
}
