fn_expected_request!(
    modules: [
        twitch_highway::extensions::ExtensionsAPI,
        twitch_highway::types::ExtensionId
    ],
    endpoint: get_extensions,
    token_type: Any,
    scopes: None,
    args: [
        ExtensionId::new("uo6dggojyb8d6soh92zknwmi5ej1q2"),
        Some("0.0.9")
    ],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/extensions?extension_id=uo6dggojyb8d6soh92zknwmi5ej1q2&extension_version=0.0.9"
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"author_name\": \"Twitch Developers\",\n      \"bits_enabled\": true,\n      \"can_install\": false,\n      \"configuration_location\": \"hosted\",\n      \"description\": \"An extension for testing all the features that we add to extensions\",\n      \"eula_tos_url\": \"\",\n      \"has_chat_support\": true,\n      \"icon_url\": \"https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/logob6c995d8-8b45-48cc-a748-b256e92ac1cd\",\n      \"icon_urls\": {\n        \"100x100\": \"https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/logob6c995d8-8b45-48cc-a748-b256e92ac1cd\",\n        \"24x24\": \"https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/taskbar905b19da-e7e5-4d8f-beb7-f543a861ac1e\",\n        \"300x200\": \"https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/discoveryd9545b2c-5474-46d7-a523-1c3835d862ce\"\n      },\n      \"id\": \"pgn0bjv51epi7eaekt53tovjnc82qo\",\n      \"name\": \"Official Developers Demo\",\n      \"privacy_policy_url\": \"\",\n      \"request_identity_link\": true,\n      \"screenshot_urls\": [\n        \"https://extensions-discovery-images.twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.8/screenshotbdec475d-3d2f-4378-b334-941dfddc897a\"\n      ],\n      \"state\": \"Released\",\n      \"subscriptions_support_level\": \"optional\",\n      \"summary\": \"Test ALL the extensions features!\",\n      \"support_email\": \"dx-extensions-test-dev@justin.tv\",\n      \"version\": \"0.0.9\",\n      \"viewer_summary\": \"Test ALL the extensions features!\",\n      \"views\": {\n        \"mobile\": {\n          \"viewer_url\": \"https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html\"\n        },\n        \"panel\": {\n          \"viewer_url\": \"https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html\",\n          \"height\": 300,\n          \"can_link_external_content\": false\n        },\n        \"video_overlay\": {\n          \"viewer_url\": \"https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html\",\n          \"can_link_external_content\": false\n        },\n        \"component\": {\n          \"viewer_url\": \"https://pgn0bjv51epi7eaekt53tovjnc82qo.ext-twitch.tv/pgn0bjv51epi7eaekt53tovjnc82qo/0.0.9/f9a0d8aae0f9dd0b2d6ef3416b96bc79/index.html\",\n          \"aspect_width\": 0,\n          \"aspect_height\": 0,\n          \"aspect_ratio_x\": 48000,\n          \"aspect_ratio_y\": 36000,\n          \"autoscale\": true,\n          \"scale_pixels\": 1024,\n          \"target_height\": 5333,\n          \"size\": 0,\n          \"zoom\": false,\n          \"zoom_pixels\": 0,\n          \"can_link_external_content\": false\n        }\n      },\n      \"allowlisted_config_urls\": [],\n      \"allowlisted_panel_urls\": []\n    }\n  ]\n}",
    module: twitch_highway::extensions::response::ExtensionsResponse,
    de: ExtensionsResponse
);
