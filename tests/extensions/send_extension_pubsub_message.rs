fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::types::BroadcasterId
    ],
    endpoint: send_extension_pubsub_message,
    token_type: Any,
    scopes: None,
    args: [
        &["broadcast"],
        "hello world!",
        BroadcasterId::new("141981764"),
        None
    ],
    json_contain: [
        "\"message\":\"hello world!\"",
        "\"broadcaster_id\":\"141981764\"",
        "\"target\":[\"broadcast\"]"
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/extensions/pubsub"
);
