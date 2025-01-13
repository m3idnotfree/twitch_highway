use twitch_highway::{extensions::request::ExtensionChatMessageRequestBody, types::BroadcasterId};

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: send_extension_chat_message,
    token_type: Any,
    scopes: None,
    args: [
        BroadcasterId::new("237757755"),
        ExtensionChatMessageRequestBody::new(
            "Hello".to_string(),
            "uo6dggojyb8d6soh92zknwmi5ej1q2".to_string(),
            "0.0.9".to_string()
        )
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/extensions/chat?broadcaster_id=237757755"
);
