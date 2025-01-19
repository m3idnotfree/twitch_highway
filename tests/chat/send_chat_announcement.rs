fn_expected_request!(
    modules: [
        twitch_highway::chat::ChatAPI,
        twitch_highway::chat::request::AnnouncementColor,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: send_chat_announcement,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageAnnouncements]),
    args: [
        BroadcasterId::new("11111"),
        ModeratorId::new("44444"),
        "Hello chat!",
        Some(AnnouncementColor::Purple)
    ],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/chat/announcements?broadcaster_id=11111&moderator_id=44444",
    json: Some("{\"message\":\"Hello chat!\",\"color\":\"purple\"}".to_string())
);
