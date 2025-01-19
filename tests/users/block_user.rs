fn_expected_request!(
    modules: [
        twitch_highway::users::UserAPI,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: block_user,
    token_type: User,
    scopes: Some(vec![Scope::UserManageBlockedUsers]),
    args: [UserId::new("198704263"), None, None],
    method: PUT,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users/blocks?target_user_id=198704263",
    json: None,
    text: None,
    urlencoded: None
);
