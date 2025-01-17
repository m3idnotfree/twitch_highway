new_fn_mock_server_f!(
    name: get_bits_leaderboard,
    oauth: {
        @user,
        module: BitsScopes,
        scopes: with_bits_leaderboard_read
    },
    api: {
        modules: [
            twitch_highway::bits::BitsAPI
        ],
        endpoint: get_bits_leaderboard,
        args: |_a| {None}
    }
);

new_fn_mock_server_f!(
    name: get_cheermotes,
    oauth: {
        @user,
        module: BitsScopes,
        scopes: with_cheermotes_read
    },
    api: {
        modules: [
            twitch_highway::bits::BitsAPI
        ],
        endpoint: get_cheermotes,
        args: |_a| {None}
    }
);

new_fn_mock_server_f!(
    name: except_extension_endpoint_get_extension_transactions,
    oauth: {
        @user,
        module: BitsScopes,
        scopes: with_extension_transactions_read
    },
    api: {
        modules: [
            twitch_highway::bits::BitsAPI,
            twitch_highway::types::ExtensionId
        ],
        endpoint: get_extension_transactions,
        args: |_a| {
            ExtensionId::new("123"),
            None,
            None
        },
        status: NOT_FOUND,
        rep: false
    }
);
