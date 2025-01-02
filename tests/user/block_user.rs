fn_expected_request!(
    api:twitch_highway::users::UserAPI,
    endpoint: block_user,
    token_type: User,
    scopes: Some(vec![Scope::UserManageBlockedUsers]),
    args: ["198704263",None,None],
    method: PUT,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users/blocks?target_user_id=198704263",
    json: None,
    text: None,
    urlencoded: None
);
