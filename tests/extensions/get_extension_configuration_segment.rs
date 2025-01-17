fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::extensions::types::Segment,
        twitch_highway::types::ExtensionId,
        twitch_highway::types::JWTToken
    ],
    endpoint: get_extension_configuration_segment,
    token_type: Any,
    scopes: None,
    args: [
        JWTToken::new("<your JWT token>".to_string()),
        None,
        ExtensionId::new("your_extension_id"),
        &[Segment::Global]
    ],
    method: GET,
    header: expected_headers!(jwt),
    url: "https://api.twitch.tv/helix/extensions/configurations?extension_id=your_extension_id&segment=global"
);

fn_expected_resopnse!(
    name: response_plain,
    payload: "{\n  \"data\": [\n    {\n      \"segment\": \"global\",\n      \"content\": \"hello config!\",\n      \"version\": \"0.0.1\"\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ConfigurationSegmentResponse,
    de: ConfigurationSegmentResponse
);

fn_expected_resopnse!(
    name: response_json,
    payload: "{\n  \"data\": [\n    {\n      \"segment\": \"global\",\n      \"content\": \"{\\\"foo\\\":\\\"bar\\\"}\",\n      \"version\": \"0.0.1\"\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ConfigurationSegmentResponse,
    de: ConfigurationSegmentResponse
);
