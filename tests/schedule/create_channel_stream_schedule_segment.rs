fn_expected_request!(
    modules: [
        twitch_highway::schedule::ScheduleAPI,
        twitch_highway::schedule::request::CreateScheduleSegmentRequest,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: create_channel_stream_schedule_segment,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageSchedule]),
    args: [
        BroadcasterId::new("141981764"),
        "2021-07-01T18:00:00Z",
        "AmErica/New_York",
        "60",
        Some(CreateScheduleSegmentRequest::new()
                .is_recurring(false)
                .category_id("509670")
                .title("TwitchDev Monthly Update // July 1, 2021")
        )
    ],
    json_contain: [
        "\"start_time\":\"2021-07-01T18:00:00Z\"",
        "\"timezone\":\"AmErica/New_York\"",
        "\"is_recurring\":false",
        "\"duration\":\"60\"",
        "\"category_id\":\"509670\"",
        "\"title\":\"TwitchDev Monthly Update // July 1, 2021\""
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": {\n    \"segments\": [\n      {\n        \"id\": \"eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=\",\n        \"start_time\": \"2021-07-01T18:00:00Z\",\n        \"end_time\": \"2021-07-01T19:00:00Z\",\n        \"title\": \"TwitchDev Monthly Update // July 1, 2021\",\n        \"canceled_until\": null,\n        \"category\": {\n            \"id\": \"509670\",\n            \"name\": \"Science & Technology\"\n        },\n        \"is_recurring\": false\n      }\n    ],\n    \"broadcaster_id\": \"141981764\",\n    \"broadcaster_name\": \"TwitchDev\",\n    \"broadcaster_login\": \"twitchdev\",\n    \"vacation\": null\n  }\n}",
    module: twitch_highway::schedule::response::ScheduleResponse,
    de: ScheduleResponse
);
