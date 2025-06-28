fn_expected_request!(
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: chat_write,
    token_type: Any,
    scopes: Some(vec![Scope::UserWriteChat, Scope::UserBot, Scope::ChannelBot]),
    args: [
        BroadcasterId::new("12826"),
        "141981764",
        "Hello, world! twitchdevHype"
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/chat/messages",
    json: Some("{\"broadcaster_id\":\"12826\",\"sender_id\":\"141981764\",\"message\":\"Hello, world! twitchdevHype\"}".to_string())
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"message_id\": \"abc-123-def\",\n      \"is_sent\": true\n    }\n  ]\n}",
    module: twitch_highway::chat::response::SendChatMessageResponse,
    de: SendChatMessageResponse

);
