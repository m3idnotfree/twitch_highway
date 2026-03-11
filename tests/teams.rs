#[macro_use]
mod common;

use twitch_highway::{
    teams::TeamsAPI,
    types::{BroadcasterId, TeamId},
};

#[tokio::test]
async fn get_channel_teams() {
    let suite = common::HttpMock::new().await;
    suite.get_channel_teams().await;

    let result = suite
        .api()
        .get_channel_teams(&BroadcasterId::from("96909659"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_teams_by_id() {
    let suite = common::HttpMock::new().await;
    suite.get_teams().await;

    let result = suite.api().get_teams(&TeamId::from("6358")).await;

    assert!(result.is_ok());
}
