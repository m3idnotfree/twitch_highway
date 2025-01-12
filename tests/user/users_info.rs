use twitch_highway::types::Id;

fn_expected_request!(
    name: id,
    api: twitch_highway::users::UserAPI,
    endpoint: users_info,
    token_type: Any,
    scopes: Some(vec![Scope::UserReadEmail]),
    args: [Some(&[Id::new("141981764")]), None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users?id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: login,
    api: twitch_highway::users::UserAPI,
    endpoint: users_info,
    token_type: Any,
    scopes: Some(vec![Scope::UserReadEmail]),
    args: [None, Some(&["twitchdev".to_string()])],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users?login=twitchdev",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_request!(
    name: users_id_login,
    api: twitch_highway::users::UserAPI,
    endpoint: users_info,
    token_type: Any,
    scopes: Some(vec![Scope::UserReadEmail]),
    args: [Some(&[Id::new("141981764")]), Some(&["twitchdev".to_string()])],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users?id=141981764&login=twitchdev",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"id\": \"141981764\",\n      \"login\": \"twitchdev\",\n      \"display_name\": \"TwitchDev\",\n      \"type\": \"\",\n      \"broadcaster_type\": \"partner\",\n      \"description\": \"Supporting third-party developers building Twitch integrations from chatbots to game integrations.\",\n      \"profile_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/8a6381c7-d0c0-4576-b179-38bd5ce1d6af-profile_image-300x300.png\",\n      \"offline_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/3f13ab61-ec78-4fe6-8481-8682cb3b0ac2-channel_offline_image-1920x1080.png\",\n      \"view_count\": 5980557,\n      \"email\": \"not-real@email.com\",\n      \"created_at\": \"2016-12-14T20:32:28Z\"\n    }\n  ]\n}",
    module: twitch_highway::users::response::UsersInfoResponse,
    de: UsersInfoResponse
);
