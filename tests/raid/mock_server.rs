new_fn_mock_server_f!(
    name: start_raid,
    oauth: {
        @user,
        module: RaidsScopes,
        scopes: with_raid_start
    },
    api: {
        modules: [
            twitch_highway::raid::RaidAPI
        ],
        endpoint: start_raid,
        args: |broadcaster_id|{
            broadcaster_id.as_str(),
            "11265581"
        }
    }
);

new_fn_mock_server_f!(
    name: cancel_raid,
    oauth: {
        @user,
        module: RaidsScopes,
        scopes: with_raid_cancel
    },
    api: {
        modules: [
            twitch_highway::raid::RaidAPI
        ],
        endpoint: cancel_raid,
        args: |broadcaster_id|{
            broadcaster_id
        },
        status: NO_CONTENT
    }
);
