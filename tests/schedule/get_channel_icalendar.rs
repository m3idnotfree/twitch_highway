fn_expected_request!(
    modules: [
        twitch_highway::schedule::ScheduleAPI,
        twitch_highway::types::BroadcasterId
    ],
    endpoint: get_channel_icalendar,
    token_type: Any,
    scopes: None,
    args: [BroadcasterId::new("141981764")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/schedule/icalendar?broadcaster_id=141981764"
);
