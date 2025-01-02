fn_expected_request!(
    api:twitch_highway::chat::ChatAPI,
    endpoint: update_user_chat_color,
    token_type: User,
    scopes: Some(vec![Scope::UserManageChatColor]),
    args: ["123",twitch_highway::chat::request::ChatColor::Blue],
    method: PUT,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/chat/color?user_id=123&color=blue",
    json: None,
    text: None,
    urlencoded: None
);
