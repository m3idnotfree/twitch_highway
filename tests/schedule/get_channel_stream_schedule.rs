use twitch_highway::{schedule::request::ChannelStreamScheduleRequest, types::BroadcasterId};

fn_expected_request!(
    api: twitch_highway::schedule::ScheduleAPI,
    endpoint: get_channel_stream_schedule,
    token_type: Any,
    scopes: None,
    args: [
        BroadcasterId::new("141981764"),
        ChannelStreamScheduleRequest::new()
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/schedule?broadcaster_id=141981764"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": {\n    \"segments\": [\n      {\n        \"id\": \"eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=\",\n        \"start_time\": \"2021-07-01T18:00:00Z\",\n        \"end_time\": \"2021-07-01T19:00:00Z\",\n        \"title\": \"TwitchDev Monthly Update // July 1, 2021\",\n        \"canceled_until\": null,\n        \"category\": {\n            \"id\": \"509670\",\n            \"name\": \"Science & Technology\"\n        },\n        \"is_recurring\": false\n      }\n    ],\n    \"broadcaster_id\": \"141981764\",\n    \"broadcaster_name\": \"TwitchDev\",\n    \"broadcaster_login\": \"twitchdev\",\n    \"vacation\": null\n  },\n  \"pagination\": {}\n}",
    module: twitch_highway::schedule::response::ChannelStreamScheduleResponse,
    de: ChannelStreamScheduleResponse
);
