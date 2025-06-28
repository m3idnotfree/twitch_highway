fn_expected_request!(
    name: set_is_enabled,
    modules: [
        twitch_highway::channel_points::ChannelPointsAPI,
        twitch_highway::channel_points::request::UpdateCustomRewardRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::CustomRewardId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_custom_reward,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRedemptions]),
    args: [
        BroadcasterId::new("274637212"),
        CustomRewardId::new("92af127c-7326-4483-a52b-b0da0be61c01"),
        Some(UpdateCustomRewardRequest::new().is_enabled(false))
    ],
    json_contain: ["\"is_enabled\":false"],
    method: PATCH,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=92af127c-7326-4483-a52b-b0da0be61c01"
);

fn_expected_resopnse!(
    name: set_is_enabled_response,
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_name\": \"torpedo09\",\n      \"broadcaster_login\": \"torpedo09\",\n      \"broadcaster_id\": \"274637212\",\n      \"id\": \"92af127c-7326-4483-a52b-b0da0be61c01\",\n      \"image\": null,\n      \"background_color\": \"#00E5CB\",\n      \"is_enabled\": false,\n      \"cost\": 30000,\n      \"title\": \"game analysis 2v2\",\n      \"prompt\": \"\",\n      \"is_user_input_required\": false,\n      \"max_per_stream_setting\": {\n        \"is_enabled\": true,\n        \"max_per_stream\": 60\n      },\n      \"max_per_user_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_user_per_stream\": 0\n      },\n      \"global_cooldown_setting\": {\n        \"is_enabled\": false,\n        \"global_cooldown_seconds\": 0\n      },\n      \"is_paused\": false,\n      \"is_in_stock\": false,\n      \"default_image\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"\n      },\n      \"should_redemptions_skip_request_queue\": true,\n      \"redemptions_redeemed_current_stream\": 60,\n      \"cooldown_expires_at\": null\n    }\n  ]\n}",
    module: twitch_highway::channel_points::response::CustomRewardsResponse,
    de: CustomRewardsResponse
);

fn_expected_request!(
    name: set_title_request,
    modules: [
        twitch_highway::channel_points::ChannelPointsAPI,
        twitch_highway::channel_points::request::UpdateCustomRewardRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::CustomRewardId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_custom_reward,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRedemptions]),
    args: [
        BroadcasterId::new("274637212"),
        CustomRewardId::new("92af127c-7326-4483-a52b-b0da0be61c01"),
        Some(UpdateCustomRewardRequest::new().title("game analysis 2v2"))
    ],
    json_contain: ["\"title\":\"game analysis 2v2\""],
    method: PATCH,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212&id=92af127c-7326-4483-a52b-b0da0be61c01"
);

fn_expected_resopnse!(
    name: set_title_response,
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_name\": \"torpedo09\",\n      \"broadcaster_login\": \"torpedo09\",\n      \"broadcaster_id\": \"274637212\",\n      \"id\": \"92af127c-7326-4483-a52b-b0da0be61c01\",\n      \"image\": null,\n      \"background_color\": \"\",\n      \"is_enabled\": false,\n      \"cost\": 30000,\n      \"title\": \"game analysis 2v2\",\n      \"prompt\": \"\",\n      \"is_user_input_required\": false,\n      \"max_per_stream_setting\": {\n        \"is_enabled\": true,\n        \"max_per_stream\": 60\n      },\n      \"max_per_user_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_user_per_stream\": 0\n      },\n      \"global_cooldown_setting\": {\n        \"is_enabled\": false,\n        \"global_cooldown_seconds\": 0\n      },\n      \"is_paused\": false,\n      \"is_in_stock\": true,\n      \"default_image\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"\n      },\n      \"should_redemptions_skip_request_queue\": true,\n      \"redemptions_redeemed_current_stream\": 60,\n      \"cooldown_expires_at\": null\n    }\n  ]\n}",
    module: twitch_highway::channel_points::response::CustomRewardsResponse,
    de: CustomRewardsResponse
);
