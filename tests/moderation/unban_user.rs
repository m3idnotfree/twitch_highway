fn_expected_request!(
    modules: [
        twitch_highway::moderation::ModerationAPI,
        twitch_highway::types::BroadcasterId,
        twitch_highway::types::ModeratorId,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: unban_user,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageBannedUsers]),
    args: [BroadcasterId::new("1234"), ModeratorId::new("5678"), UserId::new("5432")],
    method: DELETE,
    header: expected_headers!(),
    url: "https://api.twitch.tv/helix/moderation/bans?broadcaster_id=1234&moderator_id=5678&user_id=5432"
);
