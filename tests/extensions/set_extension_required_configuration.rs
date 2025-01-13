use twitch_highway::{
    extensions::request::RequiredConfiguration,
    types::{BroadcasterId, ExtensionId},
};

fn_expected_request!(
    api: twitch_highway::extensions::ExtensionsAPI,
    endpoint: set_extension_required_configuration,
    token_type: Any,
    scopes: None,
    args: [
        BroadcasterId::new("274637212"),
        RequiredConfiguration::new(
            ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2"),
            "0.0.1".to_string(),
            "RCS".to_string()
        )
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
