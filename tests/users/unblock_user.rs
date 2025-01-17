fn_expected_request!(
    modules: [
        twitch_highway::users::UserAPI,
        twitch_oauth_token::types::Scope
    ],
    endpoint: unblock_user,
    token_type: User,
    scopes: Some(vec![Scope::UserManageBlockedUsers]),
    args: ["198704263"],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/users/blocks?target_user_id=198704263",
    json: None,
    text: None,
    urlencoded: None
);
