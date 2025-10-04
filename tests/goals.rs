#![cfg(feature = "goals")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{goals::GoalsAPI, types::BroadcasterId};
use twitch_oauth_token::scope::GoalScopes;

api_test!(get_creator_goals, [&BroadcasterId::from("141981764")]);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.goals_api();
    })
    .await?;

    mock_api_get_creator_goals(&api).await?;

    Ok(())
}

async fn mock_api_get_creator_goals(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_creator_goals(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
