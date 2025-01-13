use twitch_highway::types::ExtensionId;

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: get_extension_secrets,
    token_type: Any,
    scopes: None,
    args: [ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2")],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/extensions/jwt/secrets?extension_id=uo6dggojyb8d6soh92zknwmi5ej1q2"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"format_version\": 1,\n      \"secrets\": [\n        {\n          \"content\": \"secret\",\n          \"active_at\": \"2021-03-29T06:58:40.858343036Z\",\n          \"expires_at\": \"2121-03-05T06:58:40.858343036Z\"\n        }\n      ]\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ExtensionSecretsResponse,
    de: ExtensionSecretsResponse
);
