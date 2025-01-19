new_fn_mock_server_f!(
    name: users_info,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_users_read
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI
        ],
        endpoint: users_info,
        args: |broadcaster_id|{
            Some(&[broadcaster_id.as_id()]),
            None
        }
    }
);

new_fn_mock_server_f!(
    name: update_user,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_users_manage
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI,
        ],
        endpoint: update_user,
        args: |_a|{Some("BaldAngel")}
    }
);

new_fn_mock_server_f!(
    name: block_list,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_block_list_read
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI
        ],
        endpoint: block_list,
        args: |broadcaster_id|{
            broadcaster_id,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: block_user,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_block_list_manage
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI
        ],
        endpoint: block_user,
        args: |_a|{
            "11265581",
            None,
            None
        },
        status: NO_CONTENT
    }
);

new_fn_mock_server_f!(
    name: unblock_user,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_block_list_manage
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI
        ],
        endpoint: unblock_user,
        args: |_a|{
            "11265581"
        },
        status: NO_CONTENT
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_user_extensions,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_user_extensions_read
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI,
        ],
        endpoint: user_extensions,
        args: |_a|{},
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: user_active_extensions,
    oauth: {
        @user,
        module: UsersScopes,
        scopes: with_user_extensions_manage
    },
    api: {
        modules: [
            twitch_highway::users::UserAPI,
        ],
        endpoint: user_active_extensions,
        args: |_a|{None},
        status: NOT_FOUND,
        rep: false
    }
);
