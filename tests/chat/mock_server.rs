new_fn_mock_server_f!(
    name: get_chatters,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_chatters
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: get_chatters,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            None
        }
    }
);

new_fn_mock_server_f!(
    name: channel_emotes,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_channel_emotes
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: channel_emotes,
        args: |broadcaster_id|{
            broadcaster_id
        }
    }
);

new_fn_mock_server_f!(
    name: global_emotes,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_global_emotes
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: global_emotes,
        args: |_a|{}
    }
);

new_fn_mock_server_f!(
    name: emote_sets,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_user_emotes_read
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: emote_sets,
        args: |broadcaster_id|{
            &[broadcaster_id.as_str()]
        }
    }
);

new_fn_mock_server_f!(
    name: channel_badge,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_channel_badges
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: channel_badge,
        args: |broadcaster_id|{
            broadcaster_id
        }
    }
);

new_fn_mock_server_f!(
    name: global_badge,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_global_badges
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: global_badge,
        args: |_a|{}
    }
);

new_fn_mock_server_f!(
    name: get_chat_settings,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_chat_setting_read
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: get_chat_settings,
        args: |broadcaster_id|{
            broadcaster_id,
            None
        },
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_get_shared_chat_session,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_shard_chat_session_read
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI,

        ],
        endpoint: get_shared_chat_session,
        args: |broadcaster_id|{
            broadcaster_id
        },
        //paths: ["shared_chat", "session"]
        status:NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: error_user_emotes,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_user_emotes_read
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: user_emotes,
        args: |broadcaster_id|{
            broadcaster_id.as_user(),
            None,
            None
        },
        check: true
        //paths: ["chat","emotes","user"]
    }
);

new_fn_mock_server_f!(
    name: update_chat_settings,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_chat_setting_manage
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: update_chat_settings,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            None
        },
    }
);

new_fn_mock_server_f!(
    name: error_chat_write,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_chat_write
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: chat_write,
        args: |broadcaster_id|{
            broadcaster_id,
            "ssf".to_string(),
            "hell".to_string()
        },
        check: true
        //paths: ["chat","messages"]
    }
);

new_fn_mock_server_f!(
    name: user_chat_color,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_user_color_read
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI
        ],
        endpoint: user_chat_color,
        args: |broadcaster_id|{
            &[broadcaster_id.as_user()]
        }
    }
);

new_fn_mock_server_f!(
    name: update_user_chat_color,
    oauth: {
        @user,
        module: ChatScopes,
        scopes: with_user_color_manage
    },
    api: {
        modules: [
            twitch_highway::chat::ChatAPI,
            twitch_highway::chat::request::ChatColor
        ],
        endpoint: update_user_chat_color,
        args: |broadcaster_id|{
            broadcaster_id.as_user(),
            ChatColor::Red
        },
        status:NO_CONTENT
    }
);
