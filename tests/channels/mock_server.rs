new_fn_mock_server_f!(
    name: get_channel_info,
    oauth: {
        @user,
        module: ChannelScopes,
        scopes: with_channel_info_read
    },
    api: {
        modules: [
            twitch_highway::channels::ChannelsAPI
        ],
        endpoint: get_channel_info,
        args: |broadcaster_id| {
            &[broadcaster_id]
        },
    }
);

new_fn_mock_server_f!(
    name: modify_channel_info,
    oauth: {
        @user,
        module: ChannelScopes,
        scopes: with_channel_info_manage
    },
    api: {
        modules: [
            twitch_highway::channels::ChannelsAPI
        ],
        endpoint: modify_channel_info,
        args: |broadcaster_id| {
            broadcaster_id,
            None
        },
        status: NO_CONTENT
    }
);

new_fn_mock_server_f!(
    name: get_channel_editors,
    oauth: {
        @user,
        module: ChannelScopes,
        scopes: with_channel_editors_read
    },
    api: {
        modules: [
            twitch_highway::channels::ChannelsAPI
        ],
        endpoint: get_channel_editors,
        args: |broadcaster_id| {
            broadcaster_id
        }
    }
);

new_fn_mock_server_f!(
    name: get_followed_channels,
    oauth: {
        @user,
        module: ChannelScopes,
        scopes: with_followed_channels_read
    },
    api: {
        modules: [
            twitch_highway::channels::ChannelsAPI
        ],
        endpoint: get_followed_channels,
        args: |broadcaster_id|{
            broadcaster_id.as_user(),
            None,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: get_channel_followers,
    oauth: {
        @user,
        module: ChannelScopes,
        scopes: with_channel_followers_read
    },
    api: {
        modules: [
            twitch_highway::channels::ChannelsAPI
        ],
        endpoint: get_channel_followers,
        args: |broadcaster_id| {
            None,
            broadcaster_id,
            None
        }
    }
);
