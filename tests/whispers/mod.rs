fn_expected_request!(
    api: twitch_highway::whispers::WhisperAPI,
    endpoint: send_whisper,
    token_type: User,
    scopes: Some(vec![Scope::UserManageWhispers]),
    args: ["123","456","hello"],
    json_contain: ["\"message\":\"hello\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/whispers?from_user_id=123&to_user_id=456"
);
