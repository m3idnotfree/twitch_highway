new_fn_mock_server_f!(
    name: create_clip,
    oauth: {
        @user,
        module: ClipsScopes,
        scopes: with_clip_create
    },
    api: {
        modules: [
            twitch_highway::clips::ClipsAPI
        ],
        endpoint: create_clip,
        args: |broadcaster_id|{
            broadcaster_id,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: get_clips,
    oauth: {
        @user,
        module: ClipsScopes,
        scopes: with_clip_read
    },
    api: {
        modules: [
            twitch_highway::clips::ClipsAPI,
            twitch_highway::clips::request::ClipsFilter
        ],
        endpoint: get_clips,
        args: |broadcaster_id|{
            ClipsFilter::by_broadcaster(broadcaster_id),
            None,
            None
        }
    }
);
