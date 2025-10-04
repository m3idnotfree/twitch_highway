#![cfg(feature = "schedule")]

#[macro_use]
mod common;

use anyhow::Result;
use chrono::Utc;
use chrono_tz::America::New_York;
use common::{mock_api_start, TwitchFixture};
use twitch_highway::{
    schedule::{ScheduleAPI, UpdateScheduleRequest, UpdateScheduleSegmentRequest},
    types::{BroadcasterId, Id},
};
use twitch_oauth_token::scope::ScheduleScopes;

api_test!(
    get_channel_stream_schedule,
    [&BroadcasterId::from("141981764"), None, None]
);
api_test!(send get_channel_icalendar, [&BroadcasterId::from("141981764")]);
api_test!(
    update_channel_stream_schedule,
    [
        &BroadcasterId::from("141981764"),
        Some(
            UpdateScheduleRequest::new()
                .is_vacation_enabled(true)
                .vacation_start_time("2021-05-16T00:00:00Z")
                .vacation_end_time("2021-05-23T00:00:00Z")
                .timezone("America/New_York")
        )
    ]
);
// api_test!(
//     create_channel_stream_schedule_segment,
//     [
//         &BroadcasterId::from("141981764"),
//         &"2021-07-01T18:00:00Z".parse::<DateTime<Utc>>().unwrap(),
//         // "America/New_York",
//         New_York,
//         "60",
//         Some(
//             CreateScheduleSegmentRequest::new()
//                 .is_recurring(false)
//                 .category_id("509670")
//                 .title("TwitchDev Monthly Update // July 1, 2021")
//         )
//     ]
// );
api_test!(
        update_channel_stream_schedule_segment,
        [
            &BroadcasterId::from("141981764"),
            &Id::from("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="),
            Some(
                UpdateScheduleSegmentRequest::new()
                    .duration("120")
            )
        ]
    );
api_test!(
        delete_channel_stream_schedule_segment,
        [&BroadcasterId::from("141981764"), &Id::from("eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0=")]
    );

#[tokio::test]
async fn mock_api() -> Result<()> {
    let _cmd = mock_api_start().await?;

    let api = TwitchFixture::user_access_token(|scope| {
        scope.schedule_api();
    })
    .await?;

    mock_api_get_channel_stream_schedule(&api).await?;
    mock_api_get_channel_icalendar(&api).await?;
    mock_api_update_channel_stream_schedule(&api).await?;
    mock_api_create_channel_stream_schedule_segment(&api).await?;

    // mock_api_update_channel_stream_schedule_segment(&api).await?;

    mock_api_delete_channel_stream_schedule_segment(&api).await?;
    mock_api_get_channel_stream_schedule(&api).await?;
    mock_api_get_channel_stream_schedule(&api).await?;

    Ok(())
}

async fn mock_api_get_channel_stream_schedule(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_stream_schedule(&api.selected_broadcaster_id(), None, None)
        .json()
        .await?;
    Ok(())
}
async fn mock_api_get_channel_icalendar(api: &TwitchFixture) -> Result<()> {
    api.api
        .get_channel_icalendar(&api.selected_broadcaster_id())
        .text()
        .await?;
    Ok(())
}
async fn mock_api_update_channel_stream_schedule(api: &TwitchFixture) -> Result<()> {
    api.api
        .update_channel_stream_schedule(&api.selected_broadcaster_id(), None)
        .json()
        .await?;
    Ok(())
}
async fn mock_api_create_channel_stream_schedule_segment(api: &TwitchFixture) -> Result<()> {
    let time = Utc::now();

    api.api
        .create_channel_stream_schedule_segment(
            &api.selected_broadcaster_id(),
            &time,
            New_York,
            "",
            None,
        )
        .json()
        .await?;
    Ok(())
}
// async fn mock_api_update_channel_stream_schedule_segment(api: &TwitchFixture) -> Result<()> {
//     api.api
//         .update_channel_stream_schedule_segment(
//             &api.selected_broadcaster_id(),
//             &api.selected_id(),
//             None,
//         )
//         .json()
//         .await?;
//     Ok(())
// }
async fn mock_api_delete_channel_stream_schedule_segment(api: &TwitchFixture) -> Result<()> {
    api.api
        .delete_channel_stream_schedule_segment(&api.selected_broadcaster_id(), &api.selected_id())
        .json()
        .await?;
    Ok(())
}
