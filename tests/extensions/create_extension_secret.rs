fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::types::ExtensionId
    ],
    endpoint: create_extension_secret,
    token_type: Any,
    scopes: None,
    args: [ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2"), Some(600)],
    method: POST,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/extensions/jwt/secrets?extension_id=uo6dggojyb8d6soh92zknwmi5ej1q2&delay=600"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"format_version\": 1,\n      \"secrets\": [\n        {\n          \"content\": \"old-secret\",\n          \"active_at\": \"2021-03-29T06:58:40.858343036Z\",\n          \"expires_at\": \"2021-04-22T05:21:54.99261682Z\"\n        },\n        {\n          \"content\": \"new-secret\",\n          \"active_at\": \"2021-04-22T04:16:54.996365329Z\",\n          \"expires_at\": \"2121-03-29T04:16:54.996365329Z\"\n        }\n      ]\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ExtensionSecretsResponse,
    de: ExtensionSecretsResponse
);
