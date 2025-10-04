#![cfg(feature = "polls")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    polls::{PollStatus, PollsAPI, PollsRequest},
    types::{BroadcasterId, Id, Title},
};
use twitch_oauth_token::scope::PollScopes;

api_test!(
    get_polls,
    [
        &BroadcasterId::from("141981764"),
        Some(&Id::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a")),
        None
    ]
);
api_test!(
    create_poll,
    [
        &BroadcasterId::from("141981764"),
        "Heads or Tails?",
        &[Title::new("Heads"), Title::new("Tails")],
        1800,
        Some(
            PollsRequest::new()
                .channel_points_voting_enabled(true)
                .channel_points_per_vote(100)
        )
    ]
);
api_test!(
    end_poll,
    [
        &BroadcasterId::from("141981764"),
        &Id::from("ed961efd-8a3f-4cf5-a9d0-e616c590cd2a"),
        PollStatus::TERMINATED
    ]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.polls_api();
    })
    .await?;

    mock_api_get_polls(&api).await?;
    mock_api_create_poll(&api).await?;

    Ok(())
}

async fn mock_api_get_polls(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_polls(&api.selected_broadcaster_id(), None, None)
        .json()
        .await?;
    Ok(())
}
async fn mock_api_create_poll(api: &TwitchFixture) -> Result<()> {
    api.api
        .create_poll(
            &api.selected_broadcaster_id(),
            "test",
            &[Title::new("Heads"), Title::new("Tails")],
            30,
            None,
        )
        .json()
        .await?;
    Ok(())
}
