fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::types::ExtensionId,
        twitch_highway::types::BroadcasterId
    ],
    endpoint: send_extension_chat_message,
    token_type: Any,
    scopes: None,
    args: [
        BroadcasterId::new("237757755"),
        "Hello",
        ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        "0.0.9"
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/extensions/chat?broadcaster_id=237757755"
);
