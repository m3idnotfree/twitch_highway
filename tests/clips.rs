#![cfg(feature = "clips")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    clips::{ClipsAPI, ClipsSelector},
    types::{BroadcasterId, Id, PaginationQuery},
};
use twitch_oauth_token::scope::ClipScopes;

api_test!(create_clip, [&BroadcasterId::from("44322889"), None]);
api_test!(
    get_clips,
    [
        ClipsSelector::by_ids(&[Id::from("AwkwardHelplessSalamanderSwiftRage")]),
        None,
        None
    ]
);
api_test!(extra
    get_clips,
    get_clips2,
    [
        ClipsSelector::by_broadcaster_id(&BroadcasterId::from("1234")),
        None,
        Some(PaginationQuery::new().first(5))
    ]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token_with_live(|scope| {
        scope.clips_api();
    })
    .await?;

    mock_api_create_clip(&api).await?;
    mock_api_get_clips(&api).await?;

    Ok(())
}

async fn mock_api_create_clip(api: &TwitchFixture) -> Result<()> {
    let _resp = api
        .api
        .create_clip(&api.selected_broadcaster_id(), None)
        .text()
        .await?;

    Ok(())
}
async fn mock_api_get_clips(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_clips(
            ClipsSelector::by_broadcaster_id(&api.selected_broadcaster_id()),
            None,
            None,
        )
        .json()
        .await?;
    Ok(())
}
