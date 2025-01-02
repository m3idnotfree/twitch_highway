fn_expected_request!(
    api:twitch_highway::users::UserAPI,
    endpoint: block_list,
    token_type: User,
    scopes: Some(vec![Scope::UserReadBlockedUsers]),
    args: ["141981764",None,None],
    method: GET,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users/blocks?broadcaster_id=141981764",
    json: None,
    text: None,
    urlencoded: None
);

fn_expected_resopnse!(
    payload: "{\n  \"data\": [\n    {\n      \"user_id\": \"135093069\",\n      \"user_login\": \"bluelava\",\n      \"display_name\": \"BlueLava\"\n    },\n    {\n      \"user_id\": \"27419011\",\n      \"user_login\": \"travistyoj\",\n      \"display_name\": \"TravistyOJ\"\n    }\n  ]\n}",
    module: twitch_highway::users::response::BlockUserListResponse,
    de: BlockUserListResponse
);
