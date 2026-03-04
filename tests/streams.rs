#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
mod common;

use anyhow::Result;
use common::{mock_api_start, HttpMock, TwitchFixture};
use twitch_highway::{
    streams::StreamsAPI,
    types::{BroadcasterId, UserId},
};
use twitch_oauth_token::scope::StreamScopes;

api_test!(
    get_stream_key[&BroadcasterId::from("141981764")],
    create_stream_marker[&UserId::from("123"), Some("hello, this is a marker!")]
);

api_test!(get_streams | api | { api.get_streams() });

api_test!(get_followed_streams | api | { api.get_followed_streams(&UserId::from("141981764")) });

api_test!(
    get_stream_markers | api | {
        api.get_stream_markers_by_user_id(&UserId::from("123"))
            .first(5)
    }
);

api_test!(
    get_streams as get_streams_2 | api | {
        api.get_streams().user_logins(&["cohhcarnage", "lana_lux"])
    }
);

// #[tokio::test]
// async fn mock_api() -> Result<()> {
//     let _cmd = mock_api_start().await?;
//
//     let api = TwitchFixture::user_access_token_with_live(|scope| {
//         scope.streams_api();
//     })
//     .await?;
//
//     mock_api_get_stream_key(&api).await?;
//     mock_api_get_streams(&api).await?;
//     mock_api_get_followed_streams(&api).await?;
//
//     mock_api_create_stream_marker(&api).await?;
//
//     mock_api_get_stream_markers(&api).await?;
//     mock_api_get_stream_key(&api).await?;
//
//     Ok(())
// }
//
// async fn mock_api_get_stream_key(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_stream_key(&api.selected_broadcaster_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_streams(api: &TwitchFixture) -> Result<()> {
//     api.api.get_streams().json().await?;
//     Ok(())
// }
// async fn mock_api_get_followed_streams(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_followed_streams(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_create_stream_marker(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .create_stream_marker(&api.selected_user_id(), None)
//         .json()
//         .await?;
//     Ok(())
// }
// async fn mock_api_get_stream_markers(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .get_stream_markers_by_user_id(&api.selected_user_id())
//         .json()
//         .await?;
//     Ok(())
// }
