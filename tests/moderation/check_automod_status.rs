use twitch_highway::{
    moderation::request::{CheckAutoMod, CheckAutoModStatusRequest},
    types::BroadcasterId,
};

fn_expected_request!(
    api: twitch_highway::moderation::ModerationAPI,
    endpoint: check_automod_status,
    token_type: User,
    scopes: Some(vec![Scope::ModerationRead]),
    args: [
        BroadcasterId::new("12345"),
        CheckAutoModStatusRequest::new(
            vec![
                CheckAutoMod::new(
                    "123".to_string(),
                    "Hello World!".to_string()),
                    CheckAutoMod::new("393".to_string(),"Boooooo!".to_string())
            ]
       )
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
