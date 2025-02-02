fn_expected_request!(
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: update_user_chat_color,
    token_type: User,
    scopes: Some(vec![Scope::UserManageChatColor]),
    args: [
        UserId::new("123"),
        twitch_highway::chat::request::ChatColor::Blue
    ],
    method: PUT,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/color?user_id=123&color=blue",
    json: None,
    text: None,
    urlencoded: None
);
