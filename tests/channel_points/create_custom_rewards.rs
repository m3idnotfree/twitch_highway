use twitch_highway::types::BroadcasterId;

fn_expected_request!(
    api: twitch_highway::channel_points::ChannelPointsAPI,
    endpoint: create_custom_rewards,
    token_type: User,
    scopes: Some(vec![Scope::ChannelManageRedemptions]),
    args: [
        BroadcasterId::new("274637212"),
        "game analysis 1v1",
        50000,
        None
    ],
    json_contain: ["\"title\":\"game analysis 1v1\"","\"cost\":50000"],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=274637212"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_name\": \"torpedo09\",\n      \"broadcaster_login\": \"torpedo09\",\n      \"broadcaster_id\": \"274637212\",\n      \"id\": \"afaa7e34-6b17-49f0-a19a-d1e76eaaf673\",\n      \"image\": null,\n      \"background_color\": \"#00E5CB\",\n      \"is_enabled\": true,\n      \"cost\": 50000,\n      \"title\": \"game analysis 1v1\",\n      \"prompt\": \"\",\n      \"is_user_input_required\": false,\n      \"max_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_stream\": 0\n      },\n      \"max_per_user_per_stream_setting\": {\n        \"is_enabled\": false,\n        \"max_per_user_per_stream\": 0\n      },\n      \"global_cooldown_setting\": {\n        \"is_enabled\": false,\n        \"global_cooldown_seconds\": 0\n      },\n      \"is_paused\": false,\n      \"is_in_stock\": true,\n      \"default_image\": {\n        \"url_1x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\n        \"url_2x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\n        \"url_4x\": \"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"\n      },\n      \"should_redemptions_skip_request_queue\": false,\n      \"redemptions_redeemed_current_stream\": null,\n      \"cooldown_expires_at\": null\n    }\n  ]\n}",
    module: twitch_highway::channel_points::response::CustomRewardsResponse,
    de: CustomRewardsResponse
);
