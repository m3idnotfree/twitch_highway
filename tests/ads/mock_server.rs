new_fn_mock_server_f!(
    name: start_commercial,
    oauth: {
        @user,
        module: AdsScopes,
        scopes: with_stard_commercial
    },
    api: {
        modules: [twitch_highway::ads::AdsAPI],
        endpoint: start_commercial,
        args: |a| {a, 180}
    }
);

new_fn_mock_server_f!(
    name: start_commercial_broadcaster_id_does_not_match_token,
    oauth: {
        @user,
        module: AdsScopes,
        scopes: with_stard_commercial
    },
    api: {
        modules: [twitch_highway::ads::AdsAPI],
        endpoint: start_commercial,
        args: |_a, b| {b, 180},
        status: UNAUTHORIZED,
        rep: false
    }
);

//new_fn_mock_server_f!(
//    name: get_ad_schedule,
//    oauth: {
//        @user,
//        module: AdsScopes,
//        scopes: with_ad_schedule_read
//    },
//    api: {
//        modules: [twitch_highway::ads::AdsAPI,],
//        endpoint: get_ad_schedule,
//        args: |a| {a},
//        status:NOT_ACCEPTABLE
//    }
//);

//new_fn_mock_server_f!(
//    name: snooze_next_ad,
//    oauth: {
//        @user,
//        module: AnalyticsScopes,
//        scopes: with_extension_analytics_read
//    },
//    api: {
//        modules: [
//            twitch_highway::ads::AdsAPI
//        ],
//        endpoint: get_ad_schedule,
//        args: |a|{a},
//    }
//);
