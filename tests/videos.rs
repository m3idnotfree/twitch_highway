#![cfg(feature = "videos")]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{types::VideoId, videos::VideosAPI};
use twitch_oauth_token::scope::VideoScopes;

api_test!(build
    get_videos |api| {
        api.get_videos_by_ids(&[VideoId::from("335921245")])
    }
);
api_test!(
    delete_videos,
    [&[VideoId::from("1234"), VideoId::from("9876")],]
);

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.videos_api();
    })
    .await?;

    mock_api_get_videos(&api).await?;
    // mock_api_delete_videos(&api).await?;

    Ok(())
}

pub async fn mock_api_get_videos(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_videos_by_user_id(&api.selected_user_id())
        .json()
        .await?;
    Ok(())
}

// pub async fn mock_api_delete_videos(api: &TwitchFixture) -> Result<()> {
//     api.api.delete_videos(&[api.selected_id()]).json().await?;
//     Ok(())
// }
