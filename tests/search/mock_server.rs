new_fn_mock_server_f!(
    name: search_categories,
    oauth: {
        @user,
        module: SearchScopes,
        scopes: with_categories_search
    },
    api: {
        modules: [
            twitch_highway::search::SearchAPI
        ],
        endpoint: search_categories,
        args: |_a|{"he", None}
    }
);

new_fn_mock_server_f!(
    name: search_channels,
    oauth: {
        @user,
        module: SearchScopes,
        scopes: with_channels_search
    },
    api: {
        modules: [
            twitch_highway::search::SearchAPI
        ],
        endpoint: search_channels,
        args: |_a|{"channels", None, None}
    }
);
