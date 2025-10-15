#![cfg(feature = "teams")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    teams::TeamsAPI,
    types::{BroadcasterId, TeamId},
};
use twitch_oauth_token::scope::TeamScopes;

api_test!(get_channel_teams, [&BroadcasterId::from("96909659")]);
api_test!(extra
    get_teams_by_id,
    get_teams, [&TeamId::from("6358")]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.teams_api();
    })
    .await?;

    mock_api_get_channel_teams(&api).await?;
    mock_api_get_teams(&api).await?;

    Ok(())
}

async fn mock_api_get_channel_teams(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_teams(&api.selected_broadcaster_id())
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_teams(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_teams_by_id(&TeamId::from(api.selected_broadcaster_id().to_string()))
        .text()
        .await?;
    Ok(())
}
