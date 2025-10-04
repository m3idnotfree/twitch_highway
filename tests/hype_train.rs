#![cfg(feature = "hype-train")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    hype_train::HypeTrainAPI,
    types::{BroadcasterId, PaginationQuery},
};
use twitch_oauth_token::scope::HypeTrainScopes;

api_test!(
    get_hype_train_events,
    [
        &BroadcasterId::from("270954519"),
        Some(PaginationQuery::new().first(1))
    ]
);
api_test!(get_hype_train_status, [&BroadcasterId::from("123")]);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.hype_train_api();
    })
    .await?;

    mock_api_get_hype_train_events(&api).await?;
    // mock_api_get_hype_train_status(&api).await?; // page not found

    Ok(())
}

async fn mock_api_get_hype_train_events(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_hype_train_events(&api.selected_broadcaster_id(), None)
        .json()
        .await?;

    Ok(())
}

// async fn mock_api_get_hype_train_status(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_hype_train_status(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//
//     Ok(())
// }
