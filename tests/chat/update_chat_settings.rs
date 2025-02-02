fn_expected_request!(
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::chat::request::UpdateChatSettingsRequest,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_chat_settings,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageChatSettings]),
    args: [
        BroadcasterId::new("1234"),
        ModeratorId::new("5678"),
        Some(UpdateChatSettingsRequest::default().follower_mode(false))
    ],
    method: PATCH,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/chat/settings?broadcaster_id=1234&moderator_id=5678",
    json: Some("{\"follower_mode\":false}".to_string())
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"broadcaster_id\": \"1234\",\n      \"moderator_id\": \"5678\",\n      \"slow_mode\": true,\n      \"slow_mode_wait_time\": 10,\n      \"follower_mode\": false,\n      \"follower_mode_duration\": null,\n      \"subscriber_mode\": false,\n      \"emote_mode\": false,\n      \"unique_chat_mode\": false,\n      \"non_moderator_chat_delay\": false,\n      \"non_moderator_chat_delay_duration\": null\n    }\n  ]\n}",
    module: twitch_highway::chat::response::ChatSettingResponse,
    de: ChatSettingResponse
);
