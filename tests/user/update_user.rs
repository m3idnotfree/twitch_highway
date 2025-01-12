fn_expected_request!(
    api: twitch_highway::users::UserAPI,
    endpoint: update_user,
    token_type: User,
    scopes: Some(vec![Scope::UserReadEmail, Scope::UserEdit]),
    args: [Some("BaldAngel")],
    method: PUT,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users?description=BaldAngel",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\":[{\n    \"id\": \"44322889\",\n    \"login\": \"dallas\",\n    \"display_name\": \"dallas\",\n    \"type\": \"staff\",\n    \"broadcaster_type\": \"affiliate\",\n    \"description\": \"BaldAngel\",\n    \"profile_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/4d1f36cbf1f0072d-profile_image-300x300.png\",\n    \"offline_image_url\": \"https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-2e82c1df2a464df7-1920x1080.jpeg\",\n    \"view_count\": 6995,\n    \"email\": \"not-real@email.com\",\n    \"created_at\": \"2013-06-03T19:12:02.580593Z\"\n  }]\n}",
    module: twitch_highway::users::response::UpdateUsersResponse,
    de: UpdateUsersResponse
);
