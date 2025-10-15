#![cfg(feature = "predictions")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    predictions::{PredictionStatus, PredictionsAPI},
    types::{BroadcasterId, PredictionId, Title},
};
use twitch_oauth_token::scope::PredictionScopes;

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

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.predictions_api();
    })
    .await?;

    mock_api_get_predictions(&api).await?;
    mock_api_create_prediction(&api).await?;
    mock_api_end_prediction(&api).await?;

    Ok(())
}

async fn mock_api_get_predictions(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_predictions(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}

async fn mock_api_create_prediction(api: &TwitchFixture) -> Result<()> {
    api.api
        .create_prediction(
            &api.selected_broadcaster_id(),
            "Any leeks in the stream?",
            &[
                Title::new("Yes, give it time."),
                Title::new("Definitely not."),
            ],
            120,
        )
        .json()
        .await?;
    Ok(())
}
async fn mock_api_end_prediction(api: &TwitchFixture) -> Result<()> {
    let prediction_id = PredictionId::from(api.selected_id().to_string().clone());
    api.api
        .end_prediction(
            &api.selected_broadcaster_id(),
            &prediction_id,
            PredictionStatus::CANCELED,
        )
        .json()
        .await?;
    Ok(())
}
