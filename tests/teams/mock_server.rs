new_fn_mock_server_f!(
    name: get_teams,
    oauth: {
        @user,
        module: TeamsScopes,
        scopes: with_channel_teams_read
    },
    api: {
        modules: [
            twitch_highway::teams::TeamsAPI,
            twitch_highway::teams::request::TeamSelector,
            twitch_highway::types::Id
        ],
        endpoint: get_teams,
        args: |_a|{TeamSelector::by_id(Id::new("2843"))},
    }
);

new_fn_mock_server_f!(
    name: get_channel_teams,
    oauth: {
        @user,
        module: TeamsScopes,
        scopes: with_channel_teams_read
    },
    api: {
        modules: [
            twitch_highway::teams::TeamsAPI,
            twitch_highway::types::BroadcasterId
        ],
        endpoint: get_channel_teams,
        args: |_a|{BroadcasterId::new("90221436")}
    }
);
