fn_expected_request!(
    api: twitch_highway::users::UserAPI,
    endpoint: user_extensions,
    token_type: User,
    scopes: Some(vec![Scope::UserReadBroadcast, Scope::UserEditBroadcast]),
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users/extensions/list",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"wi08ebtatdc7oj83wtl9uxwz807l8b\",\n      \"version\": \"1.1.8\",\n      \"name\": \"Streamlabs Leaderboard\",\n      \"can_activate\": true,\n      \"type\": [\n        \"panel\"\n      ]\n    },\n    {\n      \"id\": \"d4uvtfdr04uq6raoenvj7m86gdk16v\",\n      \"version\": \"2.0.2\",\n      \"name\": \"Prime Subscription and Loot Reminder\",\n      \"can_activate\": true,\n      \"type\": [\n        \"overlay\"\n      ]\n    },\n    {\n      \"id\": \"rh6jq1q334hqc2rr1qlzqbvwlfl3x0\",\n       \"version\": \"1.1.0\",\n      \"name\": \"TopClip\",\n      \"can_activate\": true,\n      \"type\": [\n        \"mobile\",\n        \"panel\"\n      ]\n    },\n    {\n      \"id\": \"zfh2irvx2jb4s60f02jq0ajm8vwgka\",\n      \"version\": \"1.0.19\",\n      \"name\": \"Streamlabs\",\n      \"can_activate\": true,\n      \"type\": [\n        \"mobile\",\n        \"overlay\"\n      ]\n    },\n    {\n      \"id\": \"lqnf3zxk0rv0g7gq92mtmnirjz2cjj\",\n      \"version\": \"0.0.1\",\n      \"name\": \"Dev Experience Test\",\n      \"can_activate\": true,\n      \"type\": [\n        \"component\",\n        \"mobile\",\n        \"panel\",\n        \"overlay\"\n      ]\n    }\n  ]\n}",
    module: twitch_highway::users::response::UserExtensionsResponse,
    de: UserExtensionsResponse
);
