new_fn_mock_server_f!(
    name: get_channel_stream_schedule,
    oauth: {
        @user,
        module: ScheduleScopes,
        scopes: with_channel_stream_schedule_read
    },
    api: {
        modules: [
            twitch_highway::schedule::ScheduleAPI
        ],
        endpoint: get_channel_stream_schedule,
        args: |broadcaster_id|{
            broadcaster_id,
            None,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: update_channel_stream_schedule,
    oauth: {
        @user,
        module: ScheduleScopes,
        scopes: with_channel_stream_schedule_manage
    },
    api: {
        modules: [
            twitch_highway::schedule::ScheduleAPI
        ],
        endpoint: update_channel_stream_schedule,
        args: |broadcaster_id|{
            broadcaster_id,
            None
        },
        status: NO_CONTENT
    }
);

new_fn_mock_server_f!(
    name: create_channel_stream_schedule_segment,
    oauth: {
        @user,
        module: ScheduleScopes,
        scopes: with_channel_stream_schedule_segment_create
    },
    api: {
        modules: [
            twitch_highway::schedule::ScheduleAPI
        ],
        endpoint: create_channel_stream_schedule_segment,
        args: |broadcaster_id|{
            broadcaster_id,
            "2024-12-18T21:07:49Z",
            "America/New_York",
            "",
            None
        }
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: BAD_REQUEST_update_channel_stream_schedule_segment,
    oauth: {
        @user,
        module: ScheduleScopes,
        scopes: with_channel_stream_schedule_segment_manage
    },
    api: {
        modules: [
            twitch_highway::schedule::ScheduleAPI,
            twitch_highway::types::Id,
        ],
        endpoint: update_channel_stream_schedule_segment,
        args: |broadcaster_id|{broadcaster_id,
             Id::new("OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDUtMjggMjE6MDc6NDcuMTU2NzIxICswMDAwIFVUQw"),
            None
        },
        status: BAD_REQUEST,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: delete_channel_stream_schedule_segment,
    oauth: {
        @user,
        module: ScheduleScopes,
        scopes: with_channel_stream_schedule_segment_delete
    },
    api: {
        modules: [
            twitch_highway::schedule::ScheduleAPI,
            twitch_highway::types::Id,
        ],
        endpoint: delete_channel_stream_schedule_segment,
        args: |broadcaster_id|{
            broadcaster_id,
            Id::new("OTAwZmNlMGEtMDU5Zi1jYWQ1LWJiZGUtZmY2MjhjOGE3ODRiXDIwMjUtMDUtMjggMjE6MDc6NDcuMTU2NzIxICswMDAwIFVUQw")
        },
        status: NO_CONTENT
    }
);
