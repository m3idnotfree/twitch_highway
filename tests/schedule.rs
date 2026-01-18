#![cfg(feature = "schedule")]

#[macro_use]
mod common;

use chrono_tz::America::New_York;
use twitch_highway::{
    schedule::ScheduleAPI,
    types::{BroadcasterId, CategoryId, SegmentId},
};

api_test!(
    get_channel_stream_schedule | api | {
        api.get_channel_stream_schedule(&BroadcasterId::from("141981764"))
    }
);

api_test!(
    get_channel_icalendar[&BroadcasterId::from("141981764")],
    send
);

api_test!(
    update_channel_stream_schedule | api | {
        api.update_channel_stream_schedule(&BroadcasterId::from("141981764"))
            .is_vacation_enabled(true)
            .vacation_start_time(&"2021-05-16T00:00:00Z".parse().unwrap())
            .vacation_end_time(&"2021-05-23T00:00:00Z".parse().unwrap())
            // .timezone("America/New_York")
            .timezone(New_York)
    }
);

api_test!(
    create_channel_stream_schedule_segment | api | {
        api.create_channel_stream_schedule_segment(
            &BroadcasterId::from("141981764"),
            &"2021-07-01T18:00:00Z".parse().unwrap(),
            New_York,
            60,
        )
        .is_recurring(false)
        .category_id(&CategoryId::from("509670"))
        .title("TwitchDev Monthly Update // July 1, 2021")
    }
);

api_test!(
    update_channel_stream_schedule_segment
        | api
        | {
            api.update_channel_stream_schedule_segment(
            &BroadcasterId::from("141981764"),
            &SegmentId::from("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="),

        )
                    .duration(120)
        }
);

api_test!(
    delete_channel_stream_schedule_segment
    [&BroadcasterId::from("141981764"), &SegmentId::from("eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0=")]
);
