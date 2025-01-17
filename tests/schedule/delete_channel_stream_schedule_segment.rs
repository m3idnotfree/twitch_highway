fn_expected_request!(
    modules: [
        twitch_highway::schedule::ScheduleAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::Id,
        twitch_oauth_token::types::Scope
    ],
    endpoint: delete_channel_stream_schedule_segment,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageSchedule]),
    args: [
        BroadcasterId::new("141981764"),
        Id::new("eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0=")
    ],
    method: DELETE,
    header: expected_headers!()
    //url: "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764&id=eyJzZWdtZW50SUQiOiI4Y2EwN2E2NC0xYTZkLTRjYWItYWE5Ni0xNjIyYzNjYWUzZDkiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyMX0="
);
