#[macro_use]
mod common;

use chrono_tz::America::New_York;
use common::HttpMock;
use twitch_highway::{
    schedule::ScheduleAPI,
    types::{BroadcasterId, CategoryId, SegmentId},
};

#[tokio::test]
async fn get_channel_stream_schedule() {
    let suite = HttpMock::new().await;
    suite.get_channel_stream_schedule().await;

    let result = suite
        .api()
        .get_channel_stream_schedule(&BroadcasterId::from("141981764"))
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_icalendar() {
    let suite = HttpMock::new().await;
    suite.get_channel_icalendar().await;

    let result = suite
        .api()
        .get_channel_icalendar(&BroadcasterId::from("141981764"))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_channel_stream_schedule() {
    let suite = HttpMock::new().await;
    suite.update_channel_stream_schedule().await;

    let result = suite
        .api()
        .update_channel_stream_schedule(&BroadcasterId::from("141981764"))
        .is_vacation_enabled(true)
        .vacation_start_time(&"2021-05-16T00:00:00Z".parse().unwrap())
        .vacation_end_time(&"2021-05-23T00:00:00Z".parse().unwrap())
        // .timezone("America/New_York")
        .timezone(New_York)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn create_channel_stream_schedule_segment() {
    let suite = HttpMock::new().await;
    suite.create_channel_stream_schedule_segment().await;

    let result = suite
        .api()
        .create_channel_stream_schedule_segment(
            &BroadcasterId::from("141981764"),
            &"2021-07-01T18:00:00Z".parse().unwrap(),
            New_York,
            60,
        )
        .is_recurring(false)
        .category_id(&CategoryId::from("509670"))
        .title("TwitchDev Monthly Update // July 1, 2021")
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn update_channel_stream_schedule_segment() {
    let suite = HttpMock::new().await;
    suite.update_channel_stream_schedule_segment().await;

    let result = suite.api()
        .update_channel_stream_schedule_segment(
            &BroadcasterId::from("141981764"),
            &SegmentId::from("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="),
        )
        .duration(120)
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_channel_stream_schedule_segment() {
    let suite = HttpMock::new().await;
    suite.delete_channel_stream_schedule_segment().await;

    let result = suite.api().delete_channel_stream_schedule_segment(&BroadcasterId::from("141981764"), &SegmentId::from("eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0=")).await;

    assert!(result.is_ok());
}
