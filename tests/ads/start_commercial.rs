fn_expected_request!(
    modules: [
        twitch_highway::ads::AdsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: start_commercial,
    token_type: User,
    scopes: Some(vec![Scope::ChannelEditCommercial]),
    args: [BroadcasterId::new("41245072"), 60],
    json_contain: ["\"broadcaster_id\":\"41245072\"","\"length\":60"],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/channels/commercial"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"length\" : 60,\n      \"message\" : \"\",\n      \"retry_after\" : 480\n    }\n  ]\n}",
    module: twitch_highway::ads::response::StartCommercialResponse,
    de: StartCommercialResponse
);
