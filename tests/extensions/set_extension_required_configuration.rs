fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ExtensionId
    ],
    endpoint: set_extension_required_configuration,
    token_type: Any,
    scopes: None,
    args: [
        BroadcasterId::new("274637212"),
        ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        "0.0.1",
        "RCS"
    ],
    json_contain: [
        "\"required_configuration\":\"RCS\"",
        "\"extension_id\":\"uo6dggojyb8d6soh92zknwmi5ej1q2\"",
        "\"extension_version\":\"0.0.1\""
    ],
    method: PUT,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/extensions/required_configuration?broadcaster_id=274637212"
);
