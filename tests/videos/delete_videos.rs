fn_expected_request!(
    modules: [
        twitch_highway::videos::VideosAPI,
        twitch_highway::types::Id,
        twitch_oauth_token::types::Scope
    ],
    endpoint: delete_videos,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageVideos]),
    args: [&[Id::new("1234"), Id::new("9876")]],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/videos?id=1234&id=9876"
);
