#[macro_use]
mod common;

use common::HttpMock;
use twitch_highway::{
    clips::ClipsAPI,
    types::{BroadcasterId, ClipId, UserId},
};

#[tokio::test]
async fn create_clip() {
    let suite = HttpMock::new().await;
    suite.create_clip().await;

    let result = suite
        .api()
        .create_clip(&BroadcasterId::from("44322889"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_clips() {
    let suite = HttpMock::new().await;
    suite.get_clips().await;

    let result = suite
        .api()
        .get_clips(&[ClipId::from("AwkwardHelplessSalamanderSwiftRage")])
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_clips_download() {
    let suite = HttpMock::new().await;
    suite.get_clips_download().await;

    let result = suite
        .api()
        .get_clips_download(
            &UserId::from("141981764"),
            &BroadcasterId::from("141981764"),
            &[
                ClipId::from("InexpensiveDistinctFoxChefFrank"),
                ClipId::from("SpinelessCloudyLeopardMcaT"),
            ],
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_clips2() {
    let suite = HttpMock::new().await;
    suite.get_clips2().await;

    let result = suite
        .api()
        .get_clips(&BroadcasterId::from("1234"))
        .first(5)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_clip_from_vod() {
    let suite = HttpMock::new().await;
    suite.create_clip_from_vod().await;

    let result = suite
        .api()
        .create_clip_from_vod(
            &UserId::from("12826"),
            &BroadcasterId::from("141981764"),
            "2277656159",
            8,
            "title",
            None,
        )
        .await;

    assert!(result.is_ok());
}
