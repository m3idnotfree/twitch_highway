new_fn_mock_server_f!(
    name: get_broadcaster_subscriptions,
    oauth: {
        @user,
        module: SubscriptionsScopes,
        scopes: with_broadcaster_subscriptions_read
    },
    api: {
        modules: [
            twitch_highway::subscriptions::SubscriptionsAPI
        ],
        endpoint: get_broadcaster_subscriptions,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            None,
            None
        }
    }
);

new_fn_mock_server_f!(
    name: check_user_subscpition,
    oauth: {
        @user,
        module: SubscriptionsScopes,
        scopes: with_user_subscription_check
    },
    api: {
        modules: [
            twitch_highway::subscriptions::SubscriptionsAPI
        ],
        endpoint: check_user_subscpition,
        args: |broadcaster_id|{
            broadcaster_id.clone(),
            broadcaster_id.as_user()
        }
    }
);
