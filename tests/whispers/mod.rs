fn_expected_request!(
    modules: [
        twitch_highway::whispers::WhisperAPI,
        twitch_oauth_token::types::Scope
    ],
    endpoint: send_whisper,
    token_type: User,
    scopes: Some(vec![Scope::UserManageWhispers]),
    args: ["123","456","hello"],
    json_contain: ["\"message\":\"hello\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/whispers?from_user_id=123&to_user_id=456"
);

new_fn_mock_server_f!(
    name: mock_server,
    oauth: {
        @user,
        module: WhispersScopes,
        scopes: with_whisper_write
    },
    api: {
        modules: [
            twitch_highway::whispers::WhisperAPI
        ],
        endpoint: send_whisper,
        args: |from, to|{from.as_str(), to.as_str(), "hello"},
        status: NO_CONTENT
    }
);
