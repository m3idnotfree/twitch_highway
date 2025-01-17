fn_expected_request!(
    modules: [
        twitch_highway::schedule::ScheduleAPI,
        twitch_highway::schedule::request::UpdateScheduleSegmentRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::Id,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_channel_stream_schedule_segment,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageSchedule]),
    args: [
        BroadcasterId::new("141981764"),
        Id::new("eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="),
        Some(UpdateScheduleSegmentRequest::new().duration("120".to_string()))
    ],
    json_contain: [
        "\"duration\":\"120\""
    ],
    method: PATCH,
    header: expected_headers!(json)
    //url: "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764&id=eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": {\n    \"segments\": [\n      {\n        \"id\": \"eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0=\",\n        \"start_time\": \"2021-07-01T18:00:00Z\",\n        \"end_time\": \"2021-07-01T20:00:00Z\",\n        \"title\": \"TwitchDev Monthly Update // July 1, 2021\",\n        \"canceled_until\": null,\n        \"category\": {\n            \"id\": \"509670\",\n            \"name\": \"Science & Technology\"\n        },\n        \"is_recurring\": false\n      }\n    ],\n    \"broadcaster_id\": \"141981764\",\n    \"broadcaster_name\": \"TwitchDev\",\n    \"broadcaster_login\": \"twitchdev\",\n    \"vacation\": null\n  }\n}",
    module: twitch_highway::schedule::response::ScheduleResponse,
    de: ScheduleResponse
);
