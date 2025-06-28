fn_expected_request!(
    modules: [
        twitch_highway::moderation::ModerationAPI,
        twitch_highway::moderation::request::CheckAutoMod,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: check_automod_status,
    token_type: User,
    scopes: Some(vec![Scope::ModerationRead]),
    args: [
        BroadcasterId::new("12345"),
        &[
            CheckAutoMod::new(
                "123".to_string(),
                "Hello World!".to_string()),
                CheckAutoMod::new("393".to_string(),"Boooooo!".to_string())
        ]
    ],
    json_contain: ["\"msg_id\":\"123\"","\"msg_text\":\"Hello World!\"","\"msg_id\":\"393\"","\"msg_text\":\"Boooooo!\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=12345"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"msg_id\": \"123\",\n      \"is_permitted\": true\n    },\n    {\n      \"msg_id\": \"393\",\n      \"is_permitted\": false\n    }\n  ]\n}",
    module: twitch_highway::moderation::response::CheckAutoModStatusResponse,
    de: CheckAutoModStatusResponse
);
