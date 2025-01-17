new_fn_mock_server_f!(
    name: get_channel_guest_star_settings,
    oauth: {
        @user,
        module: GuestStarScopes,
        scopes: with_channel_guest_star_setings_read
    },
    api: {
        modules: [
            twitch_highway::guest_star::GuestStarAPI
        ],
        endpoint: get_channel_guest_star_settings,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.to_moderator()
        },
        check:true
    }
);
