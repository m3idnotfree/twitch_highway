new_fn_mock_server_f!(
    name: get_charity_campaign,
    oauth: {
        @user,
        module: CharityScopes,
        scopes: with_charity_campaign_read
    },
    api: {
        modules: [
            twitch_highway::charity::CharityAPI
        ],
        endpoint: get_charity_campaign,
        args: |broadcaster_id|{
            broadcaster_id
        }
    }
);

new_fn_mock_server_f!(
    name: get_charity_campaign_donations,
    oauth: {
        @user,
        module: CharityScopes,
        scopes: with_charity_campaign_notations_read
    },
    api: {
        modules: [
            twitch_highway::charity::CharityAPI
        ],
        endpoint: get_charity_campaign_donations,
        args: |broadcaster_id|{
            broadcaster_id,
            None
        }
    }
);
