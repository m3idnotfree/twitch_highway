new_fn_mock_server_f!(
    name: get_stream_key,
    oauth: {
        @user,
        module: StreamsScopes,
        scopes: with_stream_key_read
    },
    api: {
        modules: [
            twitch_highway::streams::StreamsAPI
        ],
        endpoint: get_stream_key,
        args: |broadcaster_id|{broadcaster_id}
    }
);

new_fn_mock_server_f!(
    name: get_streams,
    oauth: {
        @user,
        module: StreamsScopes,
        scopes: with_streams_read
    },
    api: {
        modules: [
            twitch_highway::streams::StreamsAPI
        ],
        endpoint: get_streams,
        args: |_a|{None, None}
    }
);

new_fn_mock_server_f!(
    name: get_followed_streams,
    oauth: {
        @user,
        module: StreamsScopes,
        scopes: with_followed_streams_read
    },
    api: {
        modules: [
            twitch_highway::streams::StreamsAPI
        ],
        endpoint: get_followed_streams,
        args: |broadcaster_id|{broadcaster_id.as_user(), None}
    }
);

new_fn_mock_server_f!(
    name: create_stream_marker,
    oauth: {
        @user,
        module: StreamsScopes,
        scopes: with_stream_marker_create
    },
    api: {
        modules: [
            twitch_highway::streams::StreamsAPI
        ],
        endpoint: create_stream_marker,
        args: |broadcaster_id|{broadcaster_id.as_user(), None}
    }
);

new_fn_mock_server_f!(
    name: get_stream_marker,
    oauth: {
        @user,
        module: StreamsScopes,
        scopes: with_stream_markers_read
    },
    api: {
        modules: [
            twitch_highway::streams::StreamsAPI,
            twitch_highway::streams::request::StreamMarkerFilter
        ],
        endpoint: get_stream_marker,
        args: |broadcaster_id|{StreamMarkerFilter::by_user_id(broadcaster_id.as_user()), None}
    }
);
