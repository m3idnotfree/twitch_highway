use twitch_highway::{extensions::request::PubSubMessageRequest, types::BroadcasterId};

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: send_extension_pubsub_message,
    token_type: Any,
    scopes: None,
    args: [
        PubSubMessageRequest::new(
            vec!["broadcast".to_string()],
            "hello world!".to_string(),
            BroadcasterId::new("141981764")
        )
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
