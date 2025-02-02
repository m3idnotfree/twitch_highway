fn_expected_request!(
    modules: [
        twitch_highway::schedule::ScheduleAPI,
        twitch_highway::schedule::request::UpdateScheduleRequest,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_channel_stream_schedule,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageSchedule]),
    args: [
        BroadcasterId::new("141981764"),
        Some(UpdateScheduleRequest::new()
                .is_vacation_enabled(true)
                .vacation_start_time("2021-05-16T00:00:00Z".to_string())
                .vacation_end_time("2021-05-23T00:00:00Z".to_string())
                .timezone("America/New_York".to_string())
        )
    ],
    method: PATCH,
    header: expected_headers!()
    //url: "https://api.twitch.tv/helix/schedule/segment?broadcaster_id=141981764&id=eyJzZWdtZW50SUQiOiJlNGFjYzcyNC0zNzFmLTQwMmMtODFjYS0yM2FkYTc5NzU5ZDQiLCJpc29ZZWFyIjoyMDIxLCJpc29XZWVrIjoyNn0="
);
