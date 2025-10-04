#![cfg(feature = "raid")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{raid::RaidAPI, types::BroadcasterId};
use twitch_oauth_token::scope::RaidScopes;

api_test!(
    start_raid,
    [
        &BroadcasterId::from("12345678"),
        &BroadcasterId::from("12345678")
    ]
);
api_test!(cancel_raid, [&BroadcasterId::from("12345678")]);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.raids_api();
    })
    .await?;

    mock_api_start_raid(&api).await?;
    mock_api_cancel_raid(&api).await?;

    Ok(())
}

async fn mock_api_start_raid(api: &TwitchFixture) -> Result<()> {
    let random_user = api.get_random_user()?;
    let to_user = BroadcasterId::from(random_user.id);
    api.api
        .start_raid(&api.selected_broadcaster_id(), &to_user)
        .json()
        .await?;
    Ok(())
}
async fn mock_api_cancel_raid(api: &TwitchFixture) -> Result<()> {
    api.api
        .cancel_raid(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
