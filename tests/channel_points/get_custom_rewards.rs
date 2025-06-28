fn_expected_request!(
    modules: [
        twitch_highway::channel_points::ChannelPointsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_custom_rewards,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadRedemptions]),
    args: [BroadcasterId::new("274637212"), None, None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_name\": \"torpedo09\",\n      \"broadcaster_login\": \"torpedo09\",\n      \"broadcaster_id\": \"274637212\",\n      \"id\": \"92af127c-7326-4483-a52b-b0da0be61c01\",\n      \"image\": null,\n      \"background_color\": \"#00E5CB\",\n      \"is_enabled\": true,\n      \"cost\": 50000,\n      \"title\": \"game analysis\",\n      \"prompt\": \"\",\n      \"is_user_input_required\": false,\n      \"max_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_stream\": 0\n      },\n      \"max_per_user_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_user_per_stream\": 0\n      },\n      \"global_cooldown_setting\": {\n        \"is_enabled\": false,\n        \"global_cooldown_seconds\": 0\n      },\n      \"is_paused\": false,\n      \"is_in_stock\": true,\n      \"default_image\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"\n      },\n      \"should_redemptions_skip_request_queue\": false,\n      \"redemptions_redeemed_current_stream\": null,\n      \"cooldown_expires_at\": null\n    }\n  ]\n}",
    module: twitch_highway::channel_points::response::CustomRewardsResponse,
    de: CustomRewardsResponse
);

fn_expected_request!(
    name: only_manageable_rewards_request,
    modules: [
        twitch_highway::channel_points::ChannelPointsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_custom_rewards,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadRedemptions]),
    args: [BroadcasterId::new("274637212"), None, Some(true)],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&only_manageable_rewards=true"
);

fn_expected_resopnse!(
    name: only_manageable_rewards_response,
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_name\": \"torpedo09\",\n      \"broadcaster_id\": \"274637212\",\n      \"id\": \"92af127c-7326-4483-a52b-b0da0be61c01\",\n      \"image\": null,\n      \"background_color\": \"#00E5CB\",\n      \"is_enabled\": true,\n      \"cost\": 50000,\n      \"title\": \"game analysis\",\n      \"prompt\": \"\",\n      \"is_user_input_required\": false,\n      \"max_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_stream\": 0\n      },\n      \"max_per_user_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_user_per_stream\": 0\n      },\n      \"global_cooldown_setting\": {\n        \"is_enabled\": false,\n        \"global_cooldown_seconds\": 0\n      },\n      \"is_paused\": false,\n      \"is_in_stock\": true,\n      \"default_image\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"\n      },\n      \"should_redemptions_skip_request_queue\": false,\n      \"redemptions_redeemed_current_stream\": null,\n      \"cooldown_expires_at\": null\n    }\n  ]\n}",
    module: twitch_highway::channel_points::response::CustomRewardsResponse,
    de: CustomRewardsResponse
);

fn_expected_request!(
    name: id_request,
    modules: [
        twitch_highway::channel_points::ChannelPointsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::CustomRewardId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: get_custom_rewards,
    token_type: User,
    scopes: Some(vec![Scope::ChannelReadRedemptions]),
    args: [
        BroadcasterId::new("274637212"),
        Some(&[CustomRewardId::new("92af127c-7326-4483-a52b-b0da0be61c01")]),
        None
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=92af127c-7326-4483-a52b-b0da0be61c01"
);
