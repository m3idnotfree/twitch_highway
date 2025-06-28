new_fn_mock_server_f!(
    name: check_automod_status,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_automod_status_check
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
            twitch_highway::moderation::request::CheckAutoMod
        ],
        endpoint: check_automod_status,
        args: |broadcaster_id|{
            broadcaster_id,
            &[CheckAutoMod::new("123".to_string(),"Hello World!".to_string())]
        }
    }
);

new_fn_mock_server_f!(
    name: manage_held_automod_messages,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_held_automod_messages_manage
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
            twitch_highway::moderation::request::AutoModAction
        ],
        endpoint: manage_held_automod_messages,
        args: |broadcaster_id|{
            broadcaster_id.as_user(),
            "836013710",
            AutoModAction::ALLOW
        },
        status: NO_CONTENT
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_get_auto_mod_settings,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_automod_settings_read
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: get_auto_mod_settings,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator()
        },
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_update_auto_mod_settings,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_automod_settings_manage
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
        ],
        endpoint: update_auto_mod_settings,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            None
        },
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: get_banned_users,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_banned_users_read
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: get_banned_users,
        args: |broadcaster_id|{
            broadcaster_id,
            None,
            None
        }
    }
);

//new_fn_mock_server_f!(
//    name: ban_users,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_ban_user
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI,
//            twitch_highway::moderation::request::BanUserRequest,
//            twitch_highway::types::UserId
//        ],
//        endpoint: ban_users,
//        args: |broadcaster_id|{
//            broadcaster_id.clone(),
//            broadcaster_id.to_moderator(),
//            BanUserRequest::new(UserId::new("11265581"))
//        }
//    }
//);

//new_fn_mock_server_f!(
//    name: unban_user,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_unban_user
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI,
//            twitch_highway::types::UserId
//        ],
//        endpoint: unban_user,
//        args: |broadcaster_id|{
//            broadcaster_id.clone(),
//            broadcaster_id.to_moderator(),
//            UserId::new("11265581")
//        },
//        status: NO_CONTENT
//    }
//);

//new_fn_mock_server_f!(
//    name: get_unban_requests,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_unban_requests_read
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI,
//            twitch_highway::moderation::types::UnbanRequestStatus
//        ],
//        endpoint: get_unban_requests,
//        args: |broadcaster_id|{
//            broadcaster_id.clone(),
//            broadcaster_id.to_moderator(),
//            UnbanRequestStatus::Pending,
//            None,
//            None
//        },
//        check:true
//        //paths: ["moderation","unban_requests"]
//    }
//);

//new_fn_mock_server_f!(
//    name: resolve_unban_requests,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_unban_requests_resolve
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI,
//            twitch_highway::moderation::types::UnbanRequestStatus
//        ],
//        endpoint: resolve_unban_requests,
//        args: |broadcaster_id|{
//            broadcaster_id.clone(),
//            broadcaster_id.to_moderator(),
//            "11265581",
//            UnbanRequestStatus::Pending,
//            None
//       },
//        check: true
//        //paths: ["moderation","unban_requests"]
//    }
//);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_get_blocked_terms,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_blocked_terms_read
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: get_blocked_terms,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            None
        },
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_add_blocked_term,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_blocked_terms_add
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: add_blocked_term,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            "ff"
        },
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    #[allow(non_snake_case)]
    name: NOT_FOUND_remove_blocked_term,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_blocked_terms_remove
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
            twitch_highway::types::Id
        ],
        endpoint: remove_blocked_term,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            Id::new("35249427")
        },
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: delete_chat_messages,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_chat_messages_delete
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: delete_chat_messages,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator(),
            Some("ffus")
        },
        //check: true,
        status: NO_CONTENT,
        //rep: false
    }
);

//new_fn_mock_server_f!(
//    name: get_moderated_channels,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_moderated_channels_read
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI
//        ],
//        endpoint: get_moderated_channels,
//        args: |broadcaster_id|{
//            broadcaster_id.to_user_id(),
//            None
//        },
//        check: true
//        //paths: ["moderation","channels"]
//    }
//);

new_fn_mock_server_f!(
    name: get_moderators,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_moderators_read_as_user
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: get_moderators,
        args: |broadcaster_id|{
            broadcaster_id,
            None,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: add_channel_moderator,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_chanel_moderator_add
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
            twitch_highway::types::UserId
        ],
        endpoint: add_channel_moderator,
        args: |broadcaster_id|{
            broadcaster_id,
            UserId::new("45643680")
        },
        status: UNAUTHORIZED,
        rep: false
    }
);

//new_fn_mock_server_f!(
//    name: remove_channel_moderator,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_moderated_channels_read
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI,
//            twitch_highway::types::UserId
//        ],
//        endpoint: remove_channel_moderator,
//        args: |broadcaster_id|{
//            broadcaster_id,
//            UserId::new("45643680")
//        }
//        //paths: ["moderation","moderators"]
//        ,status: URI_TOO_LONG
//    }
//);

new_fn_mock_server_f!(
    name: get_vips,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_vips_read_as_user
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: get_vips,
        args: |broadcaster_id|{
            None,
            broadcaster_id,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: add_channel_vip,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_channel_vip_add
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
            twitch_highway::types::UserId
        ],
        endpoint: add_channel_vip,
        args: |broadcaster_id|{
            UserId::new("11265581"),
            broadcaster_id
        },
        status: UNPROCESSABLE_ENTITY,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: remove_channel_vip,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_channel_vip_remove
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI,
            twitch_highway::types::UserId
        ],
        endpoint: remove_channel_vip,
        args: |broadcaster_id|{
            UserId::new("11265581"),
            broadcaster_id
        },
        status: UNPROCESSABLE_ENTITY,
        rep: false
    }
);

//new_fn_mock_server_f!(
//    name: Broke_Response_update_shield_mode_status,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_shield_mode_status_manage
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI
//        ],
//        endpoint: update_shield_mode_status,
//        args: |broadcaster_id|{
//            broadcaster_id.clone(),
//            broadcaster_id.to_moderator(),
//            false
//        }
//        //paths: ["moderation","shield_mode"]
//        //check:true
//    }
//);

new_fn_mock_server_f!(
    name: get_shield_mode_status,
    oauth: {
        @user,
        module: ModerationScopes,
        scopes: with_shield_mode_status_read
    },
    api: {
        modules: [
            twitch_highway::moderation::ModerationAPI
        ],
        endpoint: get_shield_mode_status,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_moderator()
        },
    }
);

//new_fn_mock_server_f!(
//    name: warm_chat_user,
//    oauth: {
//        @user,
//        module: ModerationScopes,
//        scopes: with_chat_user_warn
//    },
//    api: {
//        modules: [
//            twitch_highway::moderation::ModerationAPI,
//            twitch_highway::moderation::request::WarnChatUser,
//            twitch_highway::types::UserId,
//        ],
//        endpoint: warm_chat_user,
//        args: |broadcaster_id|{
//            broadcaster_id.clone(),
//            broadcaster_id.to_moderator(),
//            vec![
//                WarnChatUser::new(UserId::new("11265581"),"no reasom".to_string()),
//                //WarnChatUser::new(UserId::new(""),"no reasom".to_string()),
//            ]
//        },
//        check: true
//        //paths: ["moderation","warnings"]
//    }
//);
