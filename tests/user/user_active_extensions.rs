fn_expected_request!(
    api: twitch_highway::users::UserAPI,
    endpoint: user_active_extensions,
    token_type: Any,
    scopes: Some(vec![Scope::UserReadBroadcast, Scope::UserEditBroadcast]),
    args: [None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users/extensions",
    json: None,
    text: None,
    urlencoded: None
);

//fn_expected_resopnse!(
//    payload: "{\n  \"data\": {\n    \"panel\": {\n      \"1\": {\n        \"active\": true,\n        \"id\": \"rh6jq1q334hqc2rr1qlzqbvwlfl3x0\",\n        \"version\": \"1.1.0\",\n        \"name\": \"TopClip\"\n      },\n      \"2\": {\n        \"active\": true,\n        \"id\": \"wi08ebtatdc7oj83wtl9uxwz807l8b\",\n        \"version\": \"1.1.8\",\n        \"name\": \"Streamlabs Leaderboard\"\n      },\n      \"3\": {\n        \"active\": true,\n        \"id\": \"naty2zwfp7vecaivuve8ef1hohh6bo\",\n        \"version\": \"1.0.9\",\n        \"name\": \"Streamlabs Stream Schedule & Countdown\"\n      }\n    },\n    \"overlay\": {\n      \"1\": {\n        \"active\": true,\n        \"id\": \"zfh2irvx2jb4s60f02jq0ajm8vwgka\",\n        \"version\": \"1.0.19\",\n        \"name\": \"Streamlabs\"\n      }\n    },\n    \"component\": {\n      \"1\": {\n        \"active\": true,\n        \"id\": \"lqnf3zxk0rv0g7gq92mtmnirjz2cjj\",\n        \"version\": \"0.0.1\",\n        \"name\": \"Dev Experience Test\",\n        \"x\": 0,\n        \"y\": 0\n      },\n      \"2\": {\n        \"active\": false\n      }\n    }\n  }\n}",
//    module: twitch_highway::users::response::UserActiveExtensionsData,
//    de: UserActiveExtensionsData
//);
