use twitch_highway::{
    extensions::types::Segment,
    types::{ExtensionId, JWTToken},
};

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
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
