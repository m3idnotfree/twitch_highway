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
            twitch_highway::types::CustomRewardId
        ],
        endpoint: delete_custom_rewards,
        args: |broadcaster_id| {
            broadcaster_id,
            CustomRewardId::new("b045196d-9ce7-4a27-a9b9-279ed341ab28")
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
            twitch_highway::channel_points::ChannelPointsAPI,
            twitch_highway::types::RewardId
        ],
        endpoint: get_custom_reward_redemption,
        args: |broadcaster_id|{
            broadcaster_id,
            RewardId::new("92af127c-7326-4483-a52b-b0da0be61c01"),
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
            twitch_highway::types::CustomRewardId
        ],
        endpoint: update_custom_reward,
        args: |broadcaster_id|{
            broadcaster_id,
            CustomRewardId::new("92af127c-7326-4483-a52b-b0da0be61c01"),
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
            twitch_highway::types::RedemptionId,
            twitch_highway::types::RewardId
        ],
        endpoint: update_redemption_status,
        args: |broadcaster_id|{
            &[RedemptionId::new("17fa2df1-ad76-4804-bfa5-a40ef63efe63")],
            broadcaster_id,
            RewardId::new("92af127c-7326-4483-a52b-b0da0be61c01"),
            RedemptionStatusQuery::new(RedemptionStatus::CANCELED)
        },
        status: NOT_FOUND,
        rep: false
    }
);
