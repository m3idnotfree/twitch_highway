new_fn_mock_server_f!(
    name: get_videos,
    oauth: {
        @user,
        module: VideosScopes,
        scopes: with_videos_read
    },
    api: {
        modules: [
            twitch_highway::videos::VideosAPI,
            twitch_highway::videos::request::VideoSelector
        ],
        endpoint: get_videos,
        args: |a| {VideoSelector::by_user_id(a.as_user()), None, None}
    }
);

new_fn_mock_server_f!(
    name: delete_videos,
    oauth: {
        @user,
        module: VideosScopes,
        scopes: with_videos_delete
    },
    api: {
        modules: [
            twitch_highway::videos::VideosAPI
        ],
        endpoint: delete_videos,
        args:|a| {&[a.as_id()]}
    }
);
