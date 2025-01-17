new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUNT_get_extension_analytics,
    oauth: {
        @user,
        module: AnalyticsScopes,
        scopes: with_extension_analytics_read
    },
    api: {
        modules: [
            twitch_highway::analytics::AnalyticsAPI
        ],
        endpoint: get_extension_analytics,
        args:|_a|{},
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUNT_get_game_analytics,
    oauth: {
        @user,
        module: AnalyticsScopes,
        scopes: with_game_analytics_read
    },
    api: {
        modules: [
            twitch_highway::analytics::AnalyticsAPI

        ],
        endpoint: get_game_analytics,
        args: |_a|{ None, None },
        status: NOT_FOUND,
        rep: false
    }
);
