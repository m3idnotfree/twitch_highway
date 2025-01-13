use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::schedule::ScheduleAPI,
    endpoint: get_channel_icalendar,
    token_type: Any,
    scopes: None,
    args: [BroadcasterId::new("141981764")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/schedule/icalendar?broadcaster_id=141981764"
);
