fn_expected_request!(
    modules: [
        twitch_highway::moderation::ModerationAPI,
        twitch_highway::moderation::request::AutoModAction,
        twitch_highway::types::UserId,
        twitch_oauth_token::types::Scope
    ],
    endpoint: manage_held_automod_messages,
    token_type: User,
    scopes: Some(vec![Scope::ModeratorManageAutomod]),
    args: [UserId::new("9327994"), "836013710", AutoModAction::ALLOW],
    json_contain: ["\"user_id\":\"9327994\"","\"msg_id\":\"836013710\"","\"action\":\"ALLOW\""],
    method: POST,
    header: expected_headers!(json),
    url: "https://api.twitch.tv/helix/moderation/automod/message"
);
