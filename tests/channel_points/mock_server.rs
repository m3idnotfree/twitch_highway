new_fn_mock_server_f!(
    name: create_custom_rewards,
    oauth: {
        @user,
        module: ChannelPointsScopes,
        scopes: with_custom_reward_manage
    },
    api: {
        modules: [
            twitch_highway::channel_points::ChannelPointsAPI
        ],
        endpoint: create_custom_rewards,
        args: |broadcaster_id| {
            broadcaster_id,
            "game analysis 1v1",
            180,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: custom_reward_not_found_for_broadcaste_delete_custom_rewards,
    oauth: {
        @user,
        module: ChannelPointsScopes,
        scopes: with_custom_reward_delete
    },
    api: {
        modules: [
            twitch_highway::channel_points::ChannelPointsAPI,
            twitch_highway::types::Id
        ],
        endpoint: delete_custom_rewards,
        args: |broadcaster_id| {
            broadcaster_id,
            Id::new("game analysis 1v1")
        },
        status: NOT_FOUND,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: get_custom_rewards,
    oauth: {
        @user,
        module: ChannelPointsScopes,
        scopes: with_custom_reward_read
    },
    api: {
        modules: [
            twitch_highway::channel_points::ChannelPointsAPI
        ],
        endpoint: get_custom_rewards,
        args: |broadcaster_id|{
            broadcaster_id,
            None,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: the_status_query_parameter_is_required_if_you_don_t_specify_the_id_query_parameter_get_custom_reward_redemption,
    oauth: {
        @user,
        module: ChannelPointsScopes,
        scopes: with_custom_reward_redemption
    },
    api: {
        modules: [
            twitch_highway::channel_points::ChannelPointsAPI
        ],
        endpoint: get_custom_reward_redemption,
        args: |broadcaster_id|{
            broadcaster_id,
            "reward_id",
            None,
            None
        },
        status: BAD_REQUEST,
        rep: false
    }
);

new_fn_mock_server_f!(
    name: update_custom_reward,
    oauth: {
        @user,
        module: ChannelPointsScopes,
        scopes: with_custom_reward_manage
    },
    api: {
        modules: [
            twitch_highway::channel_points::ChannelPointsAPI,
            twitch_highway::types::Id
        ],
        endpoint: update_custom_reward,
        args: |broadcaster_id|{
            broadcaster_id,
            Id::new("reward_id"),
            None
        }
    }
);

new_fn_mock_server_f!(
    name: update_redemption_status,
    oauth: {
        @user,
        module: ChannelPointsScopes,
        scopes: with_redemption_status_manage
    },
    api: {
        modules: [
            twitch_highway::channel_points::ChannelPointsAPI,
            twitch_highway::channel_points::request::RedemptionStatusQuery,
            twitch_highway::channel_points::types::RedemptionStatus,
            twitch_highway::types::Id
        ],
        endpoint: update_redemption_status,
        args: |broadcaster_id|{
            broadcaster_id,
            vec![Id::new("35249427")],
            "reward_id",
            RedemptionStatusQuery::new(RedemptionStatus::CANCELED)
        },
        status: NOT_FOUND,
        rep: false
    }
);
